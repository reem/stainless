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
static FAILING:     &'static str = "failing";
static BENCH:       &'static str = "bench";

/// Defines the state of a `describe!` macro as it is parsing.
struct DescribeState {
    name: Option<ast::Ident>,
    before: Option<P<ast::Block>>,
    after: Option<P<ast::Block>>,
    before_each: Option<P<ast::Block>>,
    after_each: Option<P<ast::Block>>,
    subblocks: Vec<SubBlock>
}

/// Any supported subblock.
enum SubBlock {
    TestBlock(Test),
    FailingTest(Test),
    BenchBlock(Bench),
    DescribeBlock(P<ast::Item>)
}

/// A test as a description and associated block.
struct Test {
    description: String,
    block: P<ast::Block>
}

/// A benchmark, represented as a description, an associated block,
/// and an ident for the name of the Bencher argument.
struct Bench {
    bench: P<ast::Ident>,
    description: String,
    block: P<ast::Block>
}

/// Defines the overarching `describe!` syntax extension.
///
/// All other macros in stainless are actually "fake" in the sense
/// that they are detected and expanded inside of the implementation
/// of `describe!`.
pub fn describe<'a>(cx: &'a mut base::ExtCtxt, sp: codemap::Span,
                name: ast::Ident, tokens: Vec<ast::TokenTree>) -> Box<base::MacResult + 'a> {
    // Parse a full DescribeState from the input, emitting errors if used incorrectly.
    let state = parse_describe(&mut parse::tts_to_parser(cx.parse_sess(), tokens, cx.cfg()), sp, cx, Some(name));

    // Export the new module.
    base::MacItems::new(Some(create_describe_item(state, sp, cx)).into_iter())
}

fn create_describe_item(state: DescribeState, sp: codemap::Span, cx: &mut base::ExtCtxt) -> P<ast::Item> {
    // Get the name of this mod.
    let name = state.name.clone().unwrap();

    // Create subblocks from a full DescribeState
    let subblocks = create_subblocks(state, sp, cx);

    // Get a glob import of all items in scope to the module that `describe!` is called from.
    //
    // This glob is `pub use super::*` so that nested `describe!` blocks (which will also contain
    // this glob) will be able to see all the symbols.
    let super_glob = cx.view_use_glob(sp, ast::Public, vec![cx.ident_of("super")]);

    // Generate the new module.
    cx.item_mod(sp, sp, name, vec![], vec![super_glob], subblocks)
}

impl SubBlock {
    fn to_item(self, sp: codemap::Span, blocks: (&Option<P<ast::Block>>, &Option<P<ast::Block>>),
               cx: &mut base::ExtCtxt) -> P<ast::Item> {
        match self {
            TestBlock(test) => test.to_item(false, sp, blocks, cx),
            FailingTest(test) => test.to_item(true, sp, blocks, cx),
            BenchBlock(bench) => bench.to_item(sp, cx),
            DescribeBlock(item) => item
        }
    }
}

impl Test {
    fn to_item(self, failing: bool, sp: codemap::Span,
               (before_each, after_each): (&Option<P<ast::Block>>, &Option<P<ast::Block>>),
               cx: &mut base::ExtCtxt) -> P<ast::Item> {
        let Test { description, block } = self;

        // Create the #[test] attribute.
        let test_attribute = cx.attribute(sp, cx.meta_word(sp, token::InternedString::new("test")));

        // Create the #[should_fail] attribute.
        let should_fail = cx.attribute(sp, cx.meta_word(sp, token::InternedString::new("should_fail")));

        // Create the full test body by splicing in the statements and view items of the before and
        // after blocks if they are present.
        let test_body = match (before_each, after_each) {
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

            // Add #[test] and possibly #[should_fail]
            attrs: if failing { vec![test_attribute, should_fail] } else { vec![test_attribute] },
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
            span: sp
        })
    }
}

impl Bench {
    fn to_item(self, sp: codemap::Span, cx: &mut base::ExtCtxt) -> P<ast::Item> {
        let Bench { bench, description, block } = self;

        // Create the #[bench] attribute.
        let bench_attribute = cx.attribute(sp, cx.meta_word(sp, token::InternedString::new("bench")));

        // Create the final Item that represents the benchmark.
        P(ast::Item {
            // Name it with a snake_case version of the description.
            ident: cx.ident_of(description.replace(" ", "_").as_slice()),

            // Add #[test] and possibly #[should_fail]
            attrs: vec![bench_attribute],
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemFn(
                // Takes one argument of &mut Bencher
                cx.fn_decl(vec![ast::Arg {
                    ty: quote_ty!(cx, &mut ::test::Bencher),
                    pat: quote_pat!(cx, $bench),
                    id: ast::DUMMY_NODE_ID
                }], cx.ty_nil()),

                // All the usual types.
                ast::NormalFn,
                abi::Rust,
                ast_util::empty_generics(),

                // Add the body of the function.
                block
            ),
            // Inherited visibility (not pub)
            vis: ast::Inherited,
            span: sp
        })
    }
}

fn create_subblocks(state: DescribeState, sp: codemap::Span, cx: &mut base::ExtCtxt) -> Vec<P<ast::Item>> {
    // FIXME(reem): Implement before and after.
    let (_before, _after) = (state.before, state.after);

    let blocks = (&state.before_each, &state.after_each);
    let subblocks = state.subblocks;

    subblocks.into_iter().map(|block| { block.to_item(sp, blocks, cx) }).collect()
}

fn parse_describe(parser: &mut parse::parser::Parser, sp: codemap::Span,
                  cx: &mut base::ExtCtxt, name: Option<ast::Ident>) -> DescribeState {
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
            if token::LBRACE != parser.bump_and_get() {
                parser.fatal("Expected { after the name of a describe! block.");
            }
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
            IT => { state.subblocks.push(TestBlock(parse_test(parser))) },

            // `#[should_fail]` test.
            FAILING => { state.subblocks.push(FailingTest(parse_test(parser))) },

            // #[bench] benchmark.
            BENCH => { state.subblocks.push(BenchBlock(parse_bench(parser))) }

            // Nested `describe!` block.
            DESCRIBE => {
                // Skip over the !
                match parser.bump_and_get() {
                    token::NOT => {},
                    other => parser.fatal(format!("Expected ! but found `{}`", other).as_slice())
                };

                // Parse this sublock, generate new item.
                state.subblocks.push(DescribeBlock(create_describe_item(parse_describe(parser, sp, cx, None), sp, cx)));

                // Move past closing bracket and paren.
                //
                // This has to go in here because it two is EOF on the highest-level invocation.
                match parser.bump_and_get() {
                    token::RBRACE => {},
                    other => parser.fatal(format!("Expected }} to close `describe!` but found: `{}`", other).as_slice())
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

fn parse_test(parser: &mut parse::parser::Parser) -> Test {
    // Description of this test.
    let (description, _) = parser.parse_str();

    Test {
        // Get as a String
        description: description.get().to_string(),

        // The associated block
        block: parser.parse_block()
    }
}

fn parse_bench(parser: &mut parse::parser::Parser) -> Bench {
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

