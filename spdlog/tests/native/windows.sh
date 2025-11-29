build() {
    cargo build --example native_windows --features native,source-location --verbose
    mv ./target/debug/examples/native_windows ./target/debug/examples/native_windows_srcloc
    cargo build --example native_windows --features native --verbose
}

run() {
    set -x

    # Microsoft styled CLI options start with `/` and need to be doubled to escape in bash.
    dbgview //l ./dbgview.log
    # Wait for dbgview to start up and create the log file
    while [ ! -f ./dbgview.log ]; do sleep 1; done

    ./target/debug/examples/native_windows
    ./target/debug/examples/native_windows_srcloc

    # Wait for dbgview to flush the log file
    while [ ! -s ./dbgview.log ]; do sleep 1; done
    sleep 3
    dbgview //q

    cat ./dbgview.log
    cat ./dbgview.log | grep "\[demo] \[info] info message from spdlog-rs's WinDebugSink"
    cat ./dbgview.log | grep "\[demo] \[error] error message from spdlog-rs's WinDebugSink { error_code=114514 }"
    cat ./dbgview.log | grep -E "\[demo] \[info] \[native_windows, .+.rs:[0-9]+] info message from spdlog-rs's WinDebugSink"
    cat ./dbgview.log | grep -E "\[demo] \[error] \[native_windows, .+.rs:[0-9]+] error message from spdlog-rs's WinDebugSink { error_code=114514 }"
}

$1
