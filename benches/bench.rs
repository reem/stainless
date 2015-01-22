#![allow(unstable)]
#![feature(plugin)]
#[plugin]
extern crate stainless;
extern crate test;

describe! benchmarking {
    bench "should benchmark" (bencher) {
            bencher.iter(|| 2 * 2)
    }
}

