// Copyright 2014-2016 The Stainless Developers. See the LICENSE file at the top-level directory of
// this distrubution.
//
// Licensed under the MIT license. This file may not be copied, modified, or distributed except
// according to those terms.

#![feature(plugin)]
#![plugin(stainless)]

#[cfg(test)]
mod test {
    pub fn test_helper<T: PartialEq>(x: T, y: T) {
        if x != y { panic!("Not equal.") }
    }

    describe! helpers {
        it "should be able to use helpers" {
            test_helper(7, 7);
        }
    }
}
