// Copyright 2014-2016 The Stainless Developers. See the LICENSE file at the top-level directory of
// this distrubution.
//
// Licensed under the MIT license. This file may not be copied, modified, or distributed except
// according to those terms.

#![feature(plugin)]
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
