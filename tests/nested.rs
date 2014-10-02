#![feature(phase)]
#[phase(plugin, link)]
extern crate stainless;

describe!("top level" {
    it "less specific" {
        assert_eq!(1u, 1u);
    }

    describe!("nested" {
        it "more specific" {
            assert_eq!(2u, 2u);
        }
    })
})
