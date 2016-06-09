// Copyright 2014-2016 The Stainless Developers. See the LICENSE file at the top-level directory of
// this distrubution.
//
// Licensed under the MIT license. This file may not be copied, modified, or distributed except
// according to those terms.

#![feature(plugin)]
#![plugin(stainless)]

describe! top_level {
    before_each {
        let mut foo = 1;
    }

    after_each {
        assert_eq!(foo, 4);
    }

    describe! nested {
        before_each {
            assert_eq!(foo, 1);
            foo += 1;
        }

        it "should be more specific" {
            assert_eq!(foo, 2);
            foo += 1;
        }

        after_each {
            assert_eq!(foo, 3);
            foo += 1;
        }
    }
}
