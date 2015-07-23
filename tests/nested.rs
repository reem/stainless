#![feature(plugin,const_fn)]
#![plugin(stainless)]

describe! top_level {
    it "should be less specific" {
        assert_eq!(1, 1);
    }

    describe! nested {
        it "should be more specific" {
            assert_eq!(2, 2);
        }
    }
}
