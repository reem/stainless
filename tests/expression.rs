#![feature(plugin,const_fn)]
#![plugin(stainless)]

describe! addition {
    before_each {
        let x = 5;
        let y = 6;
        for _ in 0..5 {
        }
    }

    it "should add 5 and 6 together" {
        assert_eq!(x + y, 11);
        for _ in 0..5 {
        }
    }

    after_each {
        assert_eq!(x, 5);
        assert_eq!(y, 6);
        for _ in 0..5 {
        }
    }
}
