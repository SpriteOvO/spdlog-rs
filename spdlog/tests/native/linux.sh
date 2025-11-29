build() {
    cargo build --example native_linux --features native,libsystemd,source-location --verbose
    mv ./target/debug/examples/native_linux ./target/debug/examples/native_linux_srcloc
    cargo build --example native_linux --features native,libsystemd --verbose
}

run() {
    set -x
    ./target/debug/examples/native_linux
    ./target/debug/examples/native_linux_srcloc

    journalctl --no-pager -o verbose -t native_linux
    journalctl --no-pager -o verbose -t native_linux_srcloc

    journalctl --no-pager -o json -t native_linux | jq -e -s $'.[0].MESSAGE == "[demo] [info] info message from spdlog-rs\'s JournaldSink\n"'
    journalctl --no-pager -o json -t native_linux | jq -e -s $'.[0].PRIORITY == "6" and .[0].CODE_FILE == null and .[0].CODE_LINE == null and .[0].TID != null'
    journalctl --no-pager -o json -t native_linux | jq -e -s $'.[1].MESSAGE == "[demo] [error] error message from spdlog-rs\'s JournaldSink { error_code=114514 }\n"'
    journalctl --no-pager -o json -t native_linux | jq -e -s $'.[1].PRIORITY == "3" and .[1].CODE_FILE == null and .[1].CODE_LINE == null and .[1].TID != null'

    journalctl --no-pager -o json -t native_linux_srcloc | jq -e -s $'.[0].MESSAGE == "[demo] [info] info message from spdlog-rs\'s JournaldSink\n"'
    journalctl --no-pager -o json -t native_linux_srcloc | jq -e -s $'.[0].PRIORITY == "6" and .[0].CODE_FILE == "linux.rs" and .[0].CODE_LINE != null and .[0].TID != null'
    journalctl --no-pager -o json -t native_linux_srcloc | jq -e -s $'.[1].MESSAGE == "[demo] [error] error message from spdlog-rs\'s JournaldSink { error_code=114514 }\n"'
    journalctl --no-pager -o json -t native_linux_srcloc | jq -e -s $'.[1].PRIORITY == "3" and .[1].CODE_FILE == "linux.rs" and .[1].CODE_LINE != null and .[1].TID != null'
}

$1
