// Copyright 2014-2015 The Stainless Developers. See the LICENSE file at the top-level directory of
// this distrubution.
//
// Licensed under the MIT license. This file may not be copied, modified, or distributed except
// according to those terms.

/// ## Internal Code Guide
///
/// This crate is centered around two traits, Parse and Generate,
/// which define how to parse a struct from a Parser over Rust code
/// and how to generate Rust code from a struct respectively.
///
/// All the parsing and generation of code is done through these traits.
///
/// `describe` is responsible for expanding the `describe!` macro
/// as a whole, and delegates to the Parse and Generate implementations
/// for `DescribeState`, which holds all the information necessary to
/// generate the AST for an expanded `describe!`.
///
/// The Parse implementation of DescribeState delegates to the Parse
/// implementation of Test, defined in the test module, and Bench,
/// defined in the bench module. The Generate implementation does the
/// same.
///
/// Most of the code can be understood by just walking through the
/// implementations of Parse and Generate for all the types inside
/// this crate, which can be found in the parse and generate
/// modules respectively.
///

use syntax::{ast, source_map, parse, tokenstream};
use syntax::ptr::P;
use syntax::ext::base;
use rustc_data_structures::small_vec::OneVector;

use parse::Parse;
use generate::Generate;
use test::Test;
use bench::Bench;

/// Defines the state of a `describe!` macro as it is parsing.
#[derive(Clone)]
pub struct DescribeState {
    pub name: Option<ast::Ident>,
    pub before_each: Option<P<ast::Block>>,
    pub after_each: Option<P<ast::Block>>,
    pub subblocks: Vec<SubBlock>
}

/// Any supported subblock.
#[derive(Clone)]
pub enum SubBlock {
    Test(Test),
    Bench(Bench),
    Describe(DescribeState)
}

/// Defines the overarching `describe!` syntax extension.
///
/// All other macros in stainless are actually "fake" in the sense
/// that they are detected and expanded inside of the implementation
/// of `describe!`.
pub fn describe<'a>(cx: &'a mut base::ExtCtxt, sp: source_map::Span,
                name: ast::Ident, tokens: Vec<tokenstream::TokenTree>) -> Box<base::MacResult + 'a> {
    // Parse a full DescribeState from the input, emitting errors if used incorrectly.
    let state: DescribeState = Parse::parse(&mut parse::stream_to_parser(cx.parse_sess(), tokens.into_iter().collect()), (sp, &mut*cx, Some(name)));

    // Export the new module.
    base::MacEager::items(OneVector::from_vec(vec![state.generate(sp, cx, None)]))
}
