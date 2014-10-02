#![feature(phase)]
#[phase(plugin, link)]
extern crate stainless;

describe! top_level {
    it "should be less specific" {
        assert_eq!(1u, 1u);
    }

    describe! nested {
        it "should be more specific" {
            assert_eq!(2u, 2u);
        }
    }
}
