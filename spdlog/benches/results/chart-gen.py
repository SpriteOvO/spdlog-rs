#!/usr/bin/env python3

import sys
import copy
import re
import tomli
import numpy as np
import matplotlib as mpl
import matplotlib.pyplot as plt
import matplotlib.transforms as transforms
from pathlib import Path

# Fix the hash to make the exported SVG reproducible
mpl.rcParams['svg.hashsalt'] = 'spdlog-rs'

expected_rust_unit = 'ns/iter'
expected_cpp_unit = 'logs/sec'

def unwrap_or(v, d):
    return v if v != None else d

def parse_rust_result_raw(raw: str):
    ret = []
    benches = re.sub(r'test bench_\d+_', '', raw).splitlines()
    for bench in benches:
        name, result = map(str.strip, bench.split('...'))
        is_async = name.endswith('_async')
        # Retain for 'file_async'
        if name != 'file_async':
            name = name.removesuffix('_async')

        if result == 'unavailable':
            result = None
        elif result.startswith('bench:'):
            result = result.removeprefix('bench:').strip().replace(',', '')
            median, unit, deviation = re.search(r'(.+) (.+) \(\+/- (.+)\)', result).groups()
            if unit != expected_rust_unit:
                raise ValueError(f'unexpected unit: "{unit}"')
            result = { 'unit': unit, 'median': float(median), 'deviation': float(deviation) }
        else:
            raise ValueError(f'unexpected bench result: "{result}"')
        ret.append({ 'bench': name, 'is_async': is_async, 'value': result })
    return ret

def parse_cpp_result_raw_sync(raw: str):
    ret = []
    cases = raw.split('[info] Multi threaded: ')[1:]
    for case in cases:
        case = list(filter(lambda line: '*' * 5 not in line, case.splitlines()))
        threads, messages = re.search(r'(.+) threads, (.+) messages', case[0]).groups()
        messages = messages.replace(',', '')

        benches = []
        for bench in case[1:]:
            bench = bench.replace('[info] ', '')
            name, elapsed, logs = re.search(r'(.+)\s+Elapsed: (.+) secs\s+(.+)/sec', bench).groups()
            name = name.strip()
            logs = logs.replace(',', '')
            benches.append({ 'bench': name, 'elapsed': float(elapsed), 'logs': int(logs) })
        ret.append({ 'threads': int(threads), 'messages': int(messages), 'benches': benches })
    return ret

def parse_cpp_result_raw_async(raw: str):
    config, benches = raw.split('-\n[info]\n[info] *')
    config = config.replace('[info] ', '').splitlines()[1:]
    ret = {
        'messages': config[0].removeprefix('Messages').strip().removeprefix(': '),
        'threads': config[1].removeprefix('Threads').strip().removeprefix(': '),
        'queue': config[2].removeprefix('Queue').strip().removeprefix(': '),
        'queue_memory': config[3].removeprefix('Queue memory').strip().removeprefix(': '),
        'benches': []
    }
    benches = benches.split('Queue Overflow Policy: ')[1:]
    for bench in benches:
        bench = bench.replace('[info] ', '')
        bench = list(filter(lambda line: '*' * 5 not in line and line != '[info]', bench.splitlines()))
        name = bench[0]

        iters_logs = []
        for it in bench[1:]:
            elapsed, logs = re.search(r'Elapsed: (.+) secs\s+(.+)/sec', it).groups()
            iters_logs.append(int(logs.replace(',', '')))
        avg_logs = int(sum(iters_logs) / len(iters_logs))
        ret['benches'].append({ 'bench': name, 'logs': avg_logs })
    return ret

def extract_rust_result(pkgs, bench_name):
    pkgs = copy.deepcopy(pkgs)
    for i, pkg in enumerate(pkgs):
        found = False
        for result in pkg['result']:
            if result['bench'] == bench_name:
                found = True
                pkgs[i]['result'] = result
                break
        if not found:
            raise ValueError(f'bench "{bench_name}" not found')
    return pkgs

def pkg_label(pkg):
    label = f"{pkg['name']}\n{pkg['version']}"
    features = pkg.get('features', [])
    if features:
        label += f'\n({", ".join(features)})'
    return label

def render_rust(pkgs, title, color, better, x_margin=None):
    def pkg_order(pkg):
        result = pkg['result']
        return np.inf if result['value'] == None else result['value']['median']
    def value_label(pkg):
        result = pkg['result']
        value = result['value']
        return 'Unsupported' if value == None else str(value['median']) + f' (+/- {value['deviation']})'
    pkgs.sort(key=pkg_order)
    y_label = [pkg_label(pkg) for pkg in pkgs]
    x_data = [unwrap_or(pkg['result']['value'], {}).get('median', 0.0) for pkg in pkgs]
    x_deviation = [unwrap_or(pkg['result']['value'], {}).get('deviation', 0.0) for pkg in pkgs]
    x_label = [value_label(pkg) for pkg in pkgs]

    y = np.arange(len(pkgs))
    fig, ax = plt.subplots(figsize=(10, 7))
    ax.margins(x=x_margin)

    bars = ax.barh(y, x_data, color=color, height=0.6)
    ax.bar_label(bars, x_label, padding=3)

    ax.invert_yaxis()
    ax.set_xlabel(expected_rust_unit)
    ax.set_title(title)
    ax.set_yticks(y, y_label)
    ax.legend(title=f'{better} is better', handles=[], loc='upper right', title_fontsize='large', shadow=True)
    
    [ax.get_yticklabels()[i].set_color('red') for i, _ in filter(lambda p: p[1]['name'] == 'spdlog-rs', enumerate(pkgs))]
    return fig

def render_cpp_sync(pkgs, title, better, x_margin=None):
    cases_count = len(pkgs[0]['result_sync'])
    f_title = ['' for _ in range(cases_count)]
    y_label = [[] for _ in range(cases_count)]
    x_data = [{} for _ in range(cases_count)]

    for pkg in pkgs:
        label = pkg_label(pkg)
        for i, result in enumerate(pkg['result_sync']):
            f_title[i] = f'{title}, {result["threads"]} threads, {result["messages"]} messages'
            y_label[i].append(label)
            for bench in result['benches']:
                if bench['bench'] != 'level-off':
                    if bench['bench'] not in x_data[i]:
                        x_data[i][bench['bench']] = []
                    x_data[i][bench['bench']].append(bench['logs'])

    height = 0.25
    fig, axs = plt.subplots(nrows=len(pkgs[0]['result_sync']), figsize=(10, 11))
    fig.subplots_adjust(hspace=0.4)

    for i in range(cases_count):
        y = np.arange(len(y_label[i]))
        ax = axs[i]
        ax.margins(x=x_margin)
        ax.ticklabel_format(style='plain')
        for j, (bench, data) in enumerate(x_data[i].items()):
            bars = ax.barh(y + height * j, data, height - 0.05, label=bench)
            ax.bar_label(bars, fmt='%d', padding=3)
        ax.invert_yaxis()
        ax.set_xlabel(expected_cpp_unit)
        ax.set_title(f_title[i])
        ax.set_yticks(y + height * (len(x_data[i]) - 1) / 2, y_label[i])
        ax.add_artist(ax.legend(
            title=f'{better} is better', handles=[], loc='center right',
            bbox_to_anchor=(1, 0.3), title_fontsize='large', shadow=True
        ))
        ax.legend()
        
        [ax.get_yticklabels()[i].set_color('red') for i, _ in filter(lambda p: 'spdlog-rs' in p[1], enumerate(y_label[i]))]
    return fig

def render_cpp_async(pkgs, title, better, x_margin=None):
    bench_name_mapping = {
        'Block': 'Block / block',
        'block': 'Block / block',
        'DropIncoming': 'DropIncoming / overrun',
        'overrun': 'DropIncoming / overrun',
    }
    y_label = []
    y_queue_mem = []
    x_data = {}
    f_title = ''
    for pkg in pkgs:
        y_label.append(pkg_label(pkg))
        result = pkg['result_async']
        f_title = f'{title}, {result["threads"]} threads, {result["messages"]} messages, queue: {result["queue"]}'
        y_queue_mem.append(result["queue_memory"])
        for bench in result['benches']:
            bench_name = bench_name_mapping[bench['bench']]
            if bench_name not in x_data:
                x_data[bench_name] = []
            x_data[bench_name].append(bench['logs'])

    y = np.arange(len(y_label))
    height = 0.25
    fig, ax = plt.subplots(figsize=(10, 4))

    ax.margins(x=x_margin)
    ax.ticklabel_format(style='plain')
    for i, (bench, data) in enumerate(x_data.items()):
        bars = ax.barh(y + height * i, data, height - 0.05, label=bench)
        ax.bar_label(bars, fmt='%d', padding=3)
    ax.invert_yaxis()
    ax.set_xlabel(expected_cpp_unit)
    ax.set_title(f_title)
    ax.set_yticks(y + height * (len(x_data) - 1) / 2, y_label)
    ax.add_artist(ax.legend(
        title=f'{better} is better', handles=[], loc='center right',
        bbox_to_anchor=(1, 0.32), title_fontsize='large', shadow=True
    ))
    ax.legend(title='Queue Overflow Policy')

    for i, ytick in enumerate(ax.get_yticks()):
        ax.text(0.4, ytick, f'Queue size: {y_queue_mem[i].split(' = ')[1]}', transform=transforms.blended_transform_factory(ax.transAxes, ax.transData))
    
    [ax.get_yticklabels()[i].set_color('red') for i, _ in filter(lambda p: 'spdlog-rs' in p[1], enumerate(y_label))]
    return fig

current_dir = Path(__file__).parent
data = tomli.loads((current_dir / "data.toml").read_text())

pkgs = data['rust']
for i, pkg in enumerate(pkgs):
    pkgs[i]['result'] = parse_rust_result_raw(pkg['raw'])
    del pkg['raw']
fig_rust_sync = render_rust(extract_rust_result(pkgs, 'file'), 'Log to single file - Sync', 'tab:blue', 'lower', x_margin=0.25)
fig_rust_async = render_rust(extract_rust_result(pkgs, 'file_async'), 'Log to single file - Async', 'tab:orange', 'lower', x_margin=0.22)

pkgs = data['cpp']
for i, pkg in enumerate(pkgs):
    pkgs[i]['result_sync'] = parse_cpp_result_raw_sync(pkg['raw_sync'])
    pkgs[i]['result_async'] = parse_cpp_result_raw_async(pkg['raw_async'])
    del pkg['raw_sync']
    del pkg['raw_async']
fig_cpp_sync = render_cpp_sync(pkgs, 'spdlog-rs vs C++ spdlog\nSync', 'higher', x_margin=0.1)
fig_cpp_async = render_cpp_async(pkgs, 'spdlog-rs vs C++ spdlog\nAsync', 'higher', x_margin=0.1)

def save_svg(fig, path):
    fig.savefig(path, bbox_inches='tight', metadata={'Date': None})

if any(map(lambda arg: arg == '--export', sys.argv[1:])):
    save_svg(fig_rust_sync, current_dir / 'chart-rust-sync.svg')
    save_svg(fig_rust_async, current_dir / 'chart-rust-async.svg')
    save_svg(fig_cpp_sync, current_dir / 'chart-cpp-sync.svg')
    save_svg(fig_cpp_async, current_dir / 'chart-cpp-async.svg')
else:
    plt.show()
