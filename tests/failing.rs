#![feature(plugin)]
#![plugin(stainless)]

describe! failing {
    failing "should fail" {
        panic!("should still pass");
    }
}

