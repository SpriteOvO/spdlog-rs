#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use std::sync::Arc;

use spdlog::{prelude::*, sink::*};
use test::Bencher;

fn logger(name: &str) -> Logger {
    let path = common::BENCH_LOGS_PATH.join(format!("kv_{name}.log"));
    let sink = Arc::new(
        FileSink::builder()
            .path(path)
            .truncate(true)
            .build()
            .unwrap(),
    );
    common::build_bench_logger(|b| b.sink(sink))
}

#[bench]
fn bench_kv_str(bencher: &mut Bencher) {
    let logger = logger("str");
    bencher.iter(
        || info!(logger: logger, bench_log_message!(), kv: { k1 = "v1", k2 = "v2", k3 = "v3" }),
    )
}

#[bench]
fn bench_kv_mixed(bencher: &mut Bencher) {
    let logger = logger("mixed");
    let v = vec![1];
    bencher.iter(|| info!(logger: logger, bench_log_message!(), kv: { k1 = 1, k2: = 1.0, v:? }))
}

#[bench]
fn bench_kv_many(bencher: &mut Bencher) {
    let logger = logger("12_kv");
    bencher.iter(|| {
        info!(logger: logger, bench_log_message!(), kv: {
            k1 = "v1", k2 = "v2", k3 = "v3", k4 = "v4",
            k5 = "v5", k6 = "v6", k7 = "v7", k8 = "v8",
            k9 = "v9", k10 = "v10", k11 = "v11", k12 = "v12",
        })
    })
}
