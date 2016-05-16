#![feature(plugin)]
#![plugin(stainless)]

describe! addition {
    before_each {
        let x = 5;
        let y = 6;
    }

    it "should add 5 and 6 together" {
        assert_eq!(x + y, 11);
    }

    after_each {
        assert_eq!(x, 5);
        assert_eq!(y, 6);
    }
}
