#![allow(unstable)]
#![feature(plugin)]

#[plugin] extern crate stainless;

describe! failing {
    failing "should fail" {
        panic!("should still pass");
    }
}

