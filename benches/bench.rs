#![feature(plugin, test)]
#![plugin(stainless)]

extern crate stainless;
extern crate test;

describe! benchmarking {
    bench "should benchmark" (bencher) {
            bencher.iter(|| 2 * 2)
    }
}

