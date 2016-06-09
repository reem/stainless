// Copyright 2014-2016 The Stainless Developers. See the LICENSE file at the top-level directory of
// this distrubution.
//
// Licensed under the MIT license. This file may not be copied, modified, or distributed except
// according to those terms.

#![feature(plugin)]
#![plugin(stainless)]

#[derive(Copy, Clone)]
pub struct X(i32);

#[cfg(test)]
mod test {
    // This use must be pub so that the addition sub-module can view it.
    pub use super::X;

    describe! stainless {
        it "should be able to see outer pub uses" {
            let _ = X(5);
        }
    }
}
