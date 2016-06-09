// Copyright 2014-2016 The Stainless Developers. See the LICENSE file at the top-level directory of
// this distrubution.
//
// Licensed under the MIT license. This file may not be copied, modified, or distributed except
// according to those terms.

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
