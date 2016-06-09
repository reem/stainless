// Copyright 2014-2016 The Stainless Developers. See the LICENSE file at the top-level directory of
// this distrubution.
//
// Licensed under the MIT license. This file may not be copied, modified, or distributed except
// according to those terms.

#![feature(plugin)]
#![plugin(stainless)]

describe! failing {
    failing "should fail" {
        panic!("should still pass");
    }

    failing("should still pass") "should fail with message" {
        panic!("should still pass");
    }
}
