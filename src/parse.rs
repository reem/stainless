// Copyright 2014-2016 The Stainless Developers. See the LICENSE file at the top-level directory of
// this distrubution.
//
// Licensed under the MIT license. This file may not be copied, modified, or distributed except
// according to those terms.

use syntax::{ast, codemap};
use syntax::ext::base;
use syntax::parse::token;

use syntax::ptr::P;
use syntax::parse::parser::Parser;

use test::{Test, TestConfig};
use bench::Bench;
use describe::{DescribeState, SubBlock};

/// Trait that means something can be parsed with a configuration.
pub trait Parse<Cfg> {
    /// Parse Self from a Parser and a configuration object.
    fn parse(&mut Parser, Cfg) -> Self;
}

impl Parse<TestConfig> for Test {
    fn parse(parser: &mut Parser, test_config: TestConfig) -> Test {
        // Description of this test.
        let (description, _) = parser.parse_str().ok().expect("Test should have description");

        match parser.parse_block() {
            Ok(block) => {
                return Test {
                    // Get as a String
                    description: description.to_string(),

                    // The associated block
                    block: block,

                    test_config: test_config
                }
            },
            Err(mut err) => {
                err.children.clear();
                err.emit();
                panic!("See above error");
            }
        }
    }
}

impl Parse<()> for Bench {
    fn parse(parser: &mut Parser, _: ()) -> Bench {
        // Description of this benchmark
        let (description, _) = parser.parse_str().ok().expect("Bench should have description");

        let open_delim = parser.token.clone();
        parser.bump();

        let bench_ident = match parser.parse_ident() {
            Ok(id) => id,
            Err(e) => {
                panic!("{:?}", parser.fatal(&format!("Expected `($ident)`, got err: {:?}", e)));
            }
        };

        let close_delim = parser.token.clone();
        parser.bump();

        let name = match (open_delim, bench_ident, close_delim) {
            (token::OpenDelim(token::Paren), ident, token::CloseDelim(token::Paren)) => { ident },

            (one, two, three) => {
                panic!("{:?}", parser.fatal(&format!("Expected `($ident)`, found {:?}{:?}{:?}", one, two, three)));
            }
        };

        Bench {
            description: description.to_string(),
            block: parser.parse_block().ok().unwrap(),
            bench: P(name)
        }
    }
}

const BEFORE_EACH: &'static str = "before_each";
const GIVEN:       &'static str = "given";
const AFTER_EACH:  &'static str = "after_each";
const THEN:        &'static str = "then";
const IT:          &'static str = "it";
const IGNORE:      &'static str = "ignore";
const WHEN:        &'static str = "when";
const DESCRIBE:    &'static str = "describe";
const FAILING:     &'static str = "failing";
const BENCH:       &'static str = "bench";

impl<'a, 'b> Parse<(codemap::Span, &'a mut base::ExtCtxt<'b>, Option<ast::Ident>)> for DescribeState {
    fn parse(parser: &mut Parser,
             (sp, cx, name): (codemap::Span, &'a mut base::ExtCtxt, Option<ast::Ident>)) -> DescribeState {
        let mut state = DescribeState {
            name: None,
            before_each: None,
            after_each: None,
            subblocks: vec![],
        };

        state.name = match name {
            // Top-level describe block.
            Some(name) => Some(name),
            // Nested describe block.
            None => {
                // Get the name of this describe block
                match parser.parse_ident() {
                    Ok(name) => {
                        // Moving past the opening {
                        try(parser, token::OpenDelim(token::Brace), "{ after the name of a describe! block");
                        Some(name)
                    },
                    Err(e) => {
                        panic!("{:?}", parser.fatal(&format!("Failed to parse the name of the describe block, got err: {:?}", e)));
                    }

                }
            }
        };

        // Now parse all tests and subsections:
        while parser.token != token::CloseDelim(token::Brace) && parser.token != token::Eof {
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
            let block_name = match parser.parse_ident() {
                Ok(ident) => ident.name,
                Err(e) => {
                    // technically this should never be reachable
                    panic!("{:?}", parser.fatal(&format!("Failed to parse the name of a test block, got err: {:?}", e)));
                },
            };


            match &*block_name.as_str() {
                BEFORE_EACH | GIVEN => {
                    if state.before_each.is_some() {
                        panic!("{:?}", parser.fatal("Only one `before_each` block is allowed per `describe!` block."));
                    }
                    state.before_each = Some(parser.parse_block().ok().unwrap());
                },

                AFTER_EACH | THEN => {
                    if state.after_each.is_some() {
                        panic!("{:?}", parser.fatal("Only one `after_each` block is allowed per `describe!` block."));
                    }
                    state.after_each = Some(parser.parse_block().ok().unwrap());
                },

                // Regular `#[test]`.
                IT | WHEN => { state.subblocks.push(SubBlock::Test(Parse::parse(parser, TestConfig::test()))) },

                // `#[should_panic]` or `#[should_panic(expected = "...")] test.
                FAILING => {

                    let mut fail_msg = None;
                    if parser.token == token::OpenDelim(token::Paren) {
                        parser.bump();
                        fail_msg = Some(parser.parse_str().ok().expect("Expected failing message"));
                        try(parser, token::CloseDelim(token::Paren), "unclosed failing condition paren");
                    }
                    state.subblocks.push(SubBlock::Test(Parse::parse(parser, TestConfig::failing_test(fail_msg))))
                },

                //`#[ignore]` test
                IGNORE => { state.subblocks.push(SubBlock::Test(Parse::parse(parser, TestConfig::ignored_test()))) },

                // #[bench] benchmark.
                BENCH => { state.subblocks.push(SubBlock::Bench(Parse::parse(parser, ()))) }

                // Nested `describe!` block.
                DESCRIBE => {
                    // Skip over the ! in describe!
                    try(parser, token::Not, "!");

                    // Parse this subblock, generate new item.
                    state.subblocks.push(SubBlock::Describe(Parse::parse(parser, (sp, &mut*cx, None))));

                    // Move past closing bracket and paren.
                    //
                    // This has to go in here because it is EOF on the highest-level invocation.
                    try(parser, token::CloseDelim(token::Brace), "}} to close `describe!`")
                }

                otherwise => { illegal(parser, otherwise) }
            }
        }

        state
    }
}

// checks if current token is the expected token
fn try(parser: &mut Parser, expected_token: token::Token, err: &str) {
    if parser.token != expected_token {
        panic!("{:?}", parser.fatal(&format!("Expected {}, but found `{:?}`", err, parser.token)));
    }
    parser.bump();
}

fn illegal(parser: &mut Parser, banned: &str) {
    // Illegal block name.
    let span = parser.span;
    panic!(
        "{:?}",
        parser.span_fatal(
            span,
            &format!(
                "Expected one of: `{}`, but found: `{}`",
                format!("{}, {}, {}, {}, {}, {}, {}", BEFORE_EACH, AFTER_EACH, IT, BENCH, FAILING, DESCRIBE, IGNORE),
                banned
            )
        )
    );
}
