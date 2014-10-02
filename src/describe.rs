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

fn parse_describe(mut state: DescribeState, mut parser: parse::parser::Parser) -> DescribeState {
    // First parse the name of this describe block:
    let (name, _) = parser.parse_str();
    state.name = Some(name.get().to_string());

    // Move past the opening {
    if parse::token::LBRACE != parser.bump_and_get() {
        parser.fatal("Expected { after the description of a describe! block.");
    }

    // Now parse all tests and subsections:
    while parser.token != parse::token::RBRACE {
        // Get the name of this block, must be either:
        //     - before_each
        //     - after_each
        //     - before
        //     - after
        //     - it
        //
        // Any other top-level idents are not allowed.
        let block_name = parser.parse_ident();

        match block_name.as_str() {
            BEFORE_EACH => {
                if state.before_each.is_some() { parser.fatal("Only one `before_each` block is allowed per `describe!` block.") }
                state.before_each = Some(parser.parse_block());
            },

            AFTER_EACH => {
                if state.after_each.is_some() { parser.fatal("Only one `after_each` block is allowed per `describe!` block.") }
                state.after_each = Some(parser.parse_block());
            },

            BEFORE => {
                if state.before.is_some() { parser.fatal("Only one `before` block is allowed per `describe!` block.") }
                state.before = Some(parser.parse_block());
            },

            AFTER => {
                if state.after.is_some() { parser.fatal("Only one `after` block is allowed per `describe!` block.") }
                state.after = Some(parser.parse_block());
            },

            IT => {
                // Description of this `it` block.
                let (description, _) = parser.parse_str();

                state.tests.push(Test {
                    // Get as a String
                    description: description.get().to_string(),

                    // The associated block
                    block: parser.parse_block()
                });
            },

            banned => {
                // Illegal block name.
                let span = parser.span;
                parser.span_fatal(span, format!("Expected one of: `{}`, but found: `{}`",
                    format!("{}, {}, {}, {}, {}", BEFORE_EACH, AFTER_EACH, BEFORE, AFTER, IT).as_slice(),
                    banned).as_slice());
            }
        }
    }

    state
}

