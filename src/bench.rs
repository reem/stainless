// Copyright 2014-2015 The Stainless Developers. See the LICENSE file at the top-level directory of
// this distrubution.
//
// Licensed under the MIT license. This file may not be copied, modified, or distributed except
// according to those terms.

use syntax::ast;
use syntax::ptr::P;

/// A benchmark, represented as a description, an associated block,
/// and an ident for the name of the Bencher argument.
#[derive(Clone)]
pub struct Bench {
    pub bench: P<ast::Ident>,
    pub description: String,
    pub block: P<ast::Block>
}
