#![feature(plugin,const_fn)]
#![plugin(stainless)]

describe! failing {
    failing "should fail" {
        panic!("should still pass");
    }
}

