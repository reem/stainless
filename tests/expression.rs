#![feature(plugin)]
#![plugin(stainless)]

describe! expression_at_end_of_block {
    before_each {
        let x = 5;
        let y = 6;
        let mut z = 0;
        for _ in 0..5 {
            z += 1;
        }
    }

    it "should execute expressions at ends of test blocks as statements" {
        assert_eq!(x + y, 11);
        assert_eq!(z, 5);
        for _ in 0..5 {
            z += 1;
        }
    }

    after_each {
        assert_eq!(x, 5);
        assert_eq!(y, 6);
        assert_eq!(z, 10);
        for _ in 0..5 {
            // Purposefully empty-- tests that after_each can end with loop
        }
    }
}
