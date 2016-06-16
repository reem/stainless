// Copyright 2014-2016 The Stainless Developers. See the LICENSE file at the top-level directory of
// this distrubution.
//
// Licensed under the MIT license. This file may not be copied, modified, or distributed except
// according to those terms.

#![feature(plugin, test)]
#![plugin(stainless)]

extern crate test;

describe! benchmarking {
    bench "should benchmark" (bencher) {
            bencher.iter(|| 2 * 2)
    }
}
