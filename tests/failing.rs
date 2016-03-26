#![feature(plugin,const_fn)]
#![plugin(stainless)]

describe! failing {
    failing "should fail" {
        panic!("should still pass");
    }

    failing("should still pass") "should fail with message" {
        panic!("should still pass");
    }
}
