#![feature(plugin)]
#![plugin(stainless)]

describe! ignored_tests {

    ignore "should be ignored" {
        assert!(false);
    }
}
