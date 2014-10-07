use syntax::{ast, codemap};
use syntax::ext::base;
use syntax::parse::token;

use syntax::ptr::P;
use syntax::parse::parser::Parser;

use test::Test;
use bench::Bench;
use describe::{DescribeState, TestBlock, BenchBlock, DescribeBlock};

/// Trait that means something can be parsed with a configuration.
pub trait Parse<Cfg> {
    /// Parse Self from a Parser and a configuration object.
    fn parse(&mut Parser, Cfg) -> Self;
}

impl Parse<bool> for Test {
    fn parse(parser: &mut Parser, failing: bool) -> Test {
        // Description of this test.
        let (description, _) = parser.parse_str();

        Test {
            // Get as a String
            description: description.get().to_string(),

            // The associated block
            block: parser.parse_block(),

            failing: failing
        }
    }
}

impl Parse<()> for Bench {
    fn parse(parser: &mut Parser, _: ()) -> Bench {
        // Description of this benchmark
        let (description, _) = parser.parse_str();

        let name = match (parser.bump_and_get(), parser.parse_ident(), parser.bump_and_get()) {
            (token::LPAREN, ident, token::RPAREN) => { ident },

            (one, two, three) => {
                parser.fatal(format!("Expected `($ident)`, found {}{}{}", one, two, three).as_slice())
            }
        };

        Bench {
            description: description.get().to_string(),
            block: parser.parse_block(),
            bench: P(name)
        }
    }
}

static BEFORE_EACH: &'static str = "before_each";
static AFTER_EACH:  &'static str = "after_each";
static BEFORE:      &'static str = "before";
static AFTER:       &'static str = "after";
static IT:          &'static str = "it";
static DESCRIBE:    &'static str = "describe";
static FAILING:     &'static str = "failing";
static BENCH:       &'static str = "bench";

impl<'a, 'b> Parse<(codemap::Span, &'a mut base::ExtCtxt<'b>, Option<ast::Ident>)> for DescribeState {
    fn parse(parser: &mut Parser,
             (sp, cx, name): (codemap::Span, &'a mut base::ExtCtxt, Option<ast::Ident>)) -> DescribeState {
        let mut state = DescribeState {
            name: None, before: None, after: None,
            before_each: None, after_each: None, subblocks: vec![]
        };

        state.name = match name {
            // Top-level describe block.
            Some(name) => Some(name),
            // Nested describe block.
            None => {
                // Get the name of this describe block
                let name = parser.parse_ident();
                // Move past the opening {
                try(parser, token::LBRACE, "{ after the name of a describe! block");
                Some(name)
            }
        };

        // Now parse all tests and subsections:
        while parser.token != token::RBRACE && parser.token != token::EOF {
            // Get the name of this block, must be either:
            //     - before_each
            //     - after_each
            //     - before
            //     - after
            //     - it
            //     - failing
            //     - bench
            //     - describe!
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

                // Regular `#[test]`.
                IT => { state.subblocks.push(TestBlock(Parse::parse(parser, false))) },

                // `#[should_fail]` test.
                FAILING => { state.subblocks.push(TestBlock(Parse::parse(parser, true))) },

                // #[bench] benchmark.
                BENCH => { state.subblocks.push(BenchBlock(Parse::parse(parser, ()))) }

                // Nested `describe!` block.
                DESCRIBE => {
                    // Skip over the ! in describe!
                    try(parser, token::NOT, "!");

                    // Parse this subblock, generate new item.
                    state.subblocks.push(DescribeBlock(Parse::parse(parser, (sp, &mut*cx, None))));

                    // Move past closing bracket and paren.
                    //
                    // This has to go in here because it is EOF on the highest-level invocation.
                    try(parser, token::RBRACE, "}} to close `describe!`")
                }

                otherwise => { illegal(parser, otherwise) }
            }
        }

        state
    }
}

fn try(parser: &mut Parser, token: token::Token, err: &str) {
    let real = parser.bump_and_get();
    if real != token {
        parser.fatal(format!("Expected {}, but found `{}`", err, real).as_slice())
    }
}

fn illegal(parser: &mut Parser, banned: &str) {
    // Illegal block name.
    let span = parser.span;
    parser.span_fatal(span, format!("Expected one of: `{}`, but found: `{}`",
        format!("{}, {}, {}, {}, {}, {}, {}, {}",
                BEFORE_EACH, AFTER_EACH, BEFORE, AFTER,
                IT, BENCH, FAILING, DESCRIBE).as_slice(),
        banned).as_slice());
}

