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

use syntax::{ast, codemap, parse};
use syntax::ptr::P;
use syntax::ext::base;

use parse::Parse;
use generate::Generate;
use test::Test;
use bench::Bench;

/// Defines the state of a `describe!` macro as it is parsing.
#[deriving(Clone)]
pub struct DescribeState {
    pub name: Option<ast::Ident>,
    pub before: Option<P<ast::Block>>,
    pub after: Option<P<ast::Block>>,
    pub before_each: Option<P<ast::Block>>,
    pub after_each: Option<P<ast::Block>>,
    pub subblocks: Vec<SubBlock>
}

/// Any supported subblock.
#[deriving(Clone)]
pub enum SubBlock {
    TestBlock(Test),
    BenchBlock(Bench),
    DescribeBlock(DescribeState)
}

/// Defines the overarching `describe!` syntax extension.
///
/// All other macros in stainless are actually "fake" in the sense
/// that they are detected and expanded inside of the implementation
/// of `describe!`.
pub fn describe<'a>(cx: &'a mut base::ExtCtxt, sp: codemap::Span,
                name: ast::Ident, tokens: Vec<ast::TokenTree>) -> Box<base::MacResult + 'a> {
    // Parse a full DescribeState from the input, emitting errors if used incorrectly.
    let state: DescribeState = Parse::parse(&mut parse::tts_to_parser(cx.parse_sess(), tokens, cx.cfg()), (sp, &mut*cx, Some(name)));

    // Export the new module.
    base::MacItems::new(Some(state.generate(sp, cx, ())).into_iter())
}

