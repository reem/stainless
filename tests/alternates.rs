// Copyright 2015-2016 The Stainless Developers. See the LICENSE file at the top-level directory of
// this distrubution.
//
// Licensed under the MIT license. This file may not be copied, modified, or distributed except
// according to those terms.

#![feature(plugin)]
#![plugin(stainless)]

describe! top_level {
    given {
        let mut foo = 1;
    }

    describe! nested {
        given {
            assert_eq!(foo, 1);
            foo += 1;
        }

        when "we check foo" {
            assert_eq!(foo, 2);
            foo += 1;
        }

        then {
            assert_eq!(foo, 3);
            foo += 1;
        }
    }

    then {
        assert_eq!(foo, 4);
    }
}
