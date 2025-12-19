#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use std::sync::Arc;

use test::Bencher;

include!(concat!(
    env!("OUT_DIR"),
    "/test_utils/common_for_integration_test.rs"
));
use test_utils::*;

fn init() {
    if spdlog::init_log_crate_proxy().is_ok() {
        spdlog::set_default_logger(Arc::new(build_test_logger(|b| {
            b.error_handler(|err| panic!("an error occurred: {err}"))
        })));
    }
    log::set_max_level(log::LevelFilter::max());
}

#[bench]
fn bench_log_crate_proxy(bencher: &mut Bencher) {
    init();
    bencher.iter(|| log::info!(bench_log_message!()))
}

#[bench]
fn bench_log_crate_proxy_kv(bencher: &mut Bencher) {
    init();
    bencher.iter(|| log::info!(key1 = 42, key2 = true; bench_log_message!()))
}
