#![feature(plugin)]
#![plugin(stainless)]

extern crate stainless;

describe! failing {
    failing "should fail" {
        panic!("should still pass");
    }
}

