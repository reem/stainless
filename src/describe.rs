use syntax::{abi, ast, ast_util, codemap, parse};
use syntax::parse::token;
use syntax::ptr::P;
use syntax::ext::base;
use syntax::ext::build::AstBuilder;

static BEFORE_EACH: &'static str = "before_each";
static AFTER_EACH:  &'static str = "after_each";
static BEFORE:      &'static str = "before";
static AFTER:       &'static str = "after";
static IT:          &'static str = "it";
static DESCRIBE:    &'static str = "describe";

/// Defines the state of a `describe!` macro as it is parsing.
struct DescribeState {
    name: Option<String>,
    before: Option<P<ast::Block>>,
    after: Option<P<ast::Block>>,
    before_each: Option<P<ast::Block>>,
    after_each: Option<P<ast::Block>>,
    subblocks: Vec<SubBlock>
}

enum SubBlock {
    TestBlock(Test),
    DescribeBlock(P<ast::Item>)
}

/// A test as a description and associated block.
struct Test {
    description: String,
    block: P<ast::Block>
}

/// Defines the overarching `describe!` syntax extension.
///
/// All other macros in stainless are actually "fake" in the sense
/// that they are detected and expanded inside of the implementation
/// of `describe!`.
pub fn describe(cx: &mut base::ExtCtxt, _: codemap::Span, tokens: &[ast::TokenTree]) -> Box<base::MacResult + 'static> {
    // Parse a full DescribeState from the input, emitting errors if used incorrectly.
    let state = parse_describe(&mut parse::tts_to_parser(cx.parse_sess(), tokens.to_vec(), cx.cfg()), cx);

    // Export the new module.
    base::MacItems::new(Some(create_describe_item(state, cx)).into_iter())
}

fn create_describe_item(state: DescribeState, cx: &mut base::ExtCtxt) -> P<ast::Item> {
    // Get the name of the module from the state.
    let name = cx.ident_of(state.name.clone().unwrap().replace(" ", "_").as_slice());

    // Create subblocks from a full DescribeState
    let subblocks = create_subblocks(state, cx);

    // Get a glob import of all items in scope to the module that `describe!` is called from.
    let super_glob = cx.view_use_glob(codemap::DUMMY_SP, ast::Public, vec![cx.ident_of("super")]);

    // Generate the new module.
    cx.item_mod(codemap::DUMMY_SP, codemap::DUMMY_SP, name, vec![], vec![super_glob], subblocks)
}

fn create_subblocks(state: DescribeState, cx: &mut base::ExtCtxt) -> Vec<P<ast::Item>> {
    // FIXME(reem): Implement before and after.
    let (_before, _after) = (state.before, state.after);

    let (before_each, after_each) = (state.before_each, state.after_each);
    let subblocks = state.subblocks;

    // Create the #[test] attribute.
    let test_attribute = cx.attribute(codemap::DUMMY_SP,
                                      cx.meta_word(codemap::DUMMY_SP, token::InternedString::new("test")));

    subblocks.into_iter().map(|block| {
        match block {
            TestBlock(Test { description, block }) => {
                // Create the full test body by splicing in the statements and view items of the before and
                // after blocks if they are present.
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

                // Create the final Item that represents the test.
                P(ast::Item {
                    // Name it with a snake_case version of the description.
                    ident: cx.ident_of(description.replace(" ", "_").as_slice()),

                    // Add #[test]
                    attrs: vec![test_attribute.clone()],
                    id: ast::DUMMY_NODE_ID,
                    node: ast::ItemFn(
                        // Takes no arguments and returns ()
                        cx.fn_decl(vec![], cx.ty_nil()),

                        // All the usual types.
                        ast::NormalFn,
                        abi::Rust,
                        ast_util::empty_generics(),

                        // Add the body of the function.
                        test_body
                    ),
                    // Inherited visibility (not pub)
                    vis: ast::Inherited,
                    span: codemap::DUMMY_SP
                })
            },
            DescribeBlock(item) => item
        }
    }).collect()
}

fn parse_describe(parser: &mut parse::parser::Parser, cx: &mut base::ExtCtxt) -> DescribeState {
    let mut state = DescribeState {
        name: None, before: None, after: None,
        before_each: None, after_each: None, subblocks: vec![]
    };

    // First parse the name of this describe block:
    let (name, _) = parser.parse_str();
    state.name = Some(name.get().to_string());

    // Move past the opening {
    if token::LBRACE != parser.bump_and_get() {
        parser.fatal("Expected { after the description of a describe! block.");
    }

    // Now parse all tests and subsections:
    while parser.token != token::RBRACE {
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

                state.subblocks.push(TestBlock(Test {
                    // Get as a String
                    description: description.get().to_string(),

                    // The associated block
                    block: parser.parse_block()
                }));
            },

            // Sub `describe!` block.
            DESCRIBE => {
                // Skip over the ! and (
                match (parser.bump_and_get(), parser.bump_and_get()) {
                    (token::NOT, token::LPAREN) => {},
                    (one, two) => parser.fatal(format!("Expected describe!( but found `describe{}{}`", one, two).as_slice())
                };

                // Parse this sublock, generate new item.
                state.subblocks.push(DescribeBlock(create_describe_item(parse_describe(parser, cx), cx)));

                // Move past closing bracket and paren.
                //
                // This has to go in here because it two is EOF on the highest-level invocation.
                match (parser.bump_and_get(), parser.bump_and_get()) {
                    (token::RBRACE, token::RPAREN) => {},
                    (one, two) => parser.fatal(format!("Expected }}) to close `describe!` but found: `{}{}`", one, two).as_slice())
                }
            }

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

