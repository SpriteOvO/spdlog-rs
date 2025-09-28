#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

aggregate_bench_main!();
