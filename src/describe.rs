use syntax::{ast, codemap, parse};
use syntax::ptr::P;
use syntax::ext::base;

use parse::Parse;
use generate::Generate;
use test::Test;
use bench::Bench;

/// Defines the state of a `describe!` macro as it is parsing.
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
    DescribeBlock(P<ast::Item>)
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

