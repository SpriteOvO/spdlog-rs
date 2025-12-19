build() {
    cross build --target x86_64-linux-android --example native-android --features native,android-ndk,source-location --verbose
    mv ./target/x86_64-linux-android/debug/examples/native-android ./target/x86_64-linux-android/debug/examples/native-android-srcloc
    cross build --target x86_64-linux-android --example native-android --features native,android-ndk --verbose
}

run() {
    adb root
    adb push ./target/x86_64-linux-android/debug/examples/native-android /data
    adb push ./target/x86_64-linux-android/debug/examples/native-android-srcloc /data

    adb logcat -b all -c
    adb shell /data/native-android
    adb shell /data/native-android-srcloc
    adb logcat -s "spdlog-rs-example" -d > ./logcat.log
    cat ./logcat.log
    cat ./logcat.log | grep "I spdlog-rs-example: \[demo] info message from spdlog-rs's AndroidSink"
    cat ./logcat.log | grep "E spdlog-rs-example: \[demo] error message from spdlog-rs's AndroidSink { error_code=114514 }"
    cat ./logcat.log | grep -E "I spdlog-rs-example: \[demo] \[native_android, .+.rs:[0-9]+] info message from spdlog-rs's AndroidSink"
    cat ./logcat.log | grep -E "E spdlog-rs-example: \[demo] \[native_android, .+.rs:[0-9]+] error message from spdlog-rs's AndroidSink { error_code=114514 }"
}

$1
