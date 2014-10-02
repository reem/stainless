use syntax::util::small_vector::SmallVector;

use syntax::{abi, ast, ast_util, codemap, parse};
use syntax::ptr::P;
use syntax::ext::base;
use syntax::ext::build::AstBuilder;

static BEFORE_EACH: &'static str = "before_each";
static AFTER_EACH:  &'static str = "after_each";
static BEFORE:      &'static str = "before";
static AFTER:       &'static str = "after";
static IT:          &'static str = "it";

/// Defines the state of a `describe!` macro as it is parsing.
struct DescribeState {
    name: Option<String>,
    before: Option<P<ast::Block>>,
    after: Option<P<ast::Block>>,
    before_each: Option<P<ast::Block>>,
    after_each: Option<P<ast::Block>>,
    tests: Vec<Test>
}

/// A test as a description and associated block.
struct Test {
    description: String,
    block: P<ast::Block>
}

