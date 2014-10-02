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

fn create_tests(state: DescribeState, cx: &mut base::ExtCtxt) -> Vec<P<ast::Item>> {
    let name = state.name.unwrap().replace(" ", "_");

    // FIXME(reem): Implement before and after.
    let (_before, _after) = (state.before, state.after);

    let (before_each, after_each) = (state.before_each, state.after_each);
    let tests = state.tests;

    let test_attribute = cx.attribute(codemap::DUMMY_SP,
                                      cx.meta_word(codemap::DUMMY_SP, parse::token::InternedString::new("test")));

    tests.into_iter().map(|Test { description, block }| {
        let test_body = match (&before_each, &after_each) {
            (&None, &None) => block,

            (&Some(ref before), &None) => {
                P(ast::Block {
                    view_items: before.view_items + block.view_items,
                    stmts: before.stmts + block.stmts,
                    ..block.deref().clone()
                })
            },

            (&None, &Some(ref after)) => {
                P(ast::Block {
                    view_items: block.view_items + after.view_items,
                    stmts: block.stmts + after.stmts,
                    ..block.deref().clone()
                })
            },

            (&Some(ref before), &Some(ref after)) => {
                P(ast::Block {
                    view_items: before.view_items + block.view_items + after.view_items,
                    stmts: before.stmts + block.stmts + after.stmts,
                    ..block.deref().clone()
                })
            }
        };

        let mut test_name = name.clone();
        test_name.push_str("_");
        test_name.push_str(description.replace(" ", "_").as_slice());

        P(ast::Item {
            ident: cx.ident_of(test_name.as_slice()),
            attrs: vec![test_attribute.clone()],
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemFn(
                cx.fn_decl(vec![], cx.ty_nil()),
                ast::NormalFn,
                abi::Rust,
                ast_util::empty_generics(),
                test_body
            ),
            vis: ast::Inherited,
            span: codemap::DUMMY_SP
        })
    }).collect()
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

