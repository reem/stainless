#![feature(plugin,const_fn,test)]
#![plugin(stainless)]

extern crate test;

describe! benchmarking {
    bench "should benchmark" (bencher) {
            bencher.iter(|| 2 * 2)
    }
}
