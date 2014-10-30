#![feature(phase)]
#[phase(plugin, link)]
extern crate stainless;

describe! failing {
    failing "should fail" {
        panic!("should still pass");
    }
}

