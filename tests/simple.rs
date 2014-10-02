#![feature(phase)]
#[phase(plugin, link)]
extern crate stainless;

describe!("addition" {
    before_each {
        let x = 5u; let y = 6u;
    }

    it "should add 5 and 6 together" {
        assert_eq!(x + y, 11);
    }
})

