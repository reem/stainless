use syntax::{ast, abi, ast_util, codemap};
use syntax::ptr::P;
use syntax::ext::base;
use syntax::parse::token;

use syntax::ext::build::AstBuilder;

use test::Test;
use bench::Bench;

use describe::{DescribeState, SubBlock};

/// Trait meaning something can be turned into an ast::Item with configuration.
pub trait Generate<Cfg> {
    /// Turn Self into an ast::Item with a configuration object.
    fn generate(self, codemap::Span, &mut base::ExtCtxt, Cfg) -> P<ast::Item>;
}

impl<'a> Generate<&'a DescribeState> for Test {
    fn generate(self, sp: codemap::Span, cx: &mut base::ExtCtxt, state: &'a DescribeState) -> P<ast::Item> {
        let Test { description, block, failing } = self;

        // Create the #[test] attribute.
        let test_attribute = cx.attribute(sp, cx.meta_word(sp, token::InternedString::new("test")));

        // Create the #[should_fail] attribute.
        let should_fail = cx.attribute(sp, cx.meta_word(sp, token::InternedString::new("should_fail")));

        let non_snake_word = cx.meta_word(sp, token::InternedString::new("non_snake_case"));
        let allow_non_snake_case = cx.meta_list(sp, token::InternedString::new("allow"),
                                                vec![non_snake_word]);
        let allow_non_snake_case = cx.attribute(sp, allow_non_snake_case);

        // Create the full test body by splicing in the statements and view items of the before and
        // after blocks if they are present.
        let test_body = match (&state.before_each, &state.after_each) {
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

        // Constructing attributes:
        // #[test] - no way without it
        // #[allow(non_snake_case_attr)] as description may contain upper case
        // #[should_fail] if specified
        let mut attrs = vec![test_attribute, allow_non_snake_case];
        if failing {
            attrs.push(should_fail);
        }

        // Create the final Item that represents the test.
        P(ast::Item {
            // Name it with a snake_case version of the description.
            ident: cx.ident_of(description.replace(" ", "_").as_slice()),
            attrs: attrs,
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemFn(
                // Takes no arguments and returns ()
                cx.fn_decl(vec![], cx.ty(sp, ast::TyTup(vec![]))),

                // All the usual types.
                ast::Unsafety::Normal,
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

impl Generate<()> for Bench {
    fn generate(self, sp: codemap::Span, cx: &mut base::ExtCtxt, _: ()) -> P<ast::Item> {
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
                }], cx.ty(sp, ast::TyTup(vec![]))),

                // All the usual types.
                ast::Unsafety::Normal,
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

impl<'a> Generate<&'a DescribeState> for SubBlock {
    fn generate(self, sp: codemap::Span, cx: &mut base::ExtCtxt, state: &'a DescribeState) -> P<ast::Item> {
        match self {
            SubBlock::Test(test) => test.generate(sp, cx, state),
            SubBlock::Bench(bench) => bench.generate(sp, cx, ()),
            SubBlock::Describe(item) => item.generate(sp, cx, Some(state))
        }
    }
}

impl<'a> Generate<Option<&'a DescribeState>> for DescribeState {
    fn generate(mut self, sp: codemap::Span, cx: &mut base::ExtCtxt,
                state: Option<&'a DescribeState>) -> P<ast::Item> {
        // Get the name of this mod.
        let name = self.name.clone().unwrap();

        if let Some(state) = state {
            if let Some(ref parent) = state.before_each {
                self.before_each = match self.before_each {
                    Some(ref now) => Some(P(ast::Block {
                        view_items: parent.view_items + now.view_items,
                        stmts: parent.stmts + now.stmts,
                        ..now.deref().clone()
                    })),
                    None => Some(P(parent.deref().clone()))
                };
            }

            if let Some(ref parent) = state.after_each {
                self.after_each = match self.after_each {
                    Some(ref now) => Some(P(ast::Block {
                        view_items: now.view_items + parent.view_items,
                        stmts: now.stmts + parent.stmts,
                        ..now.deref().clone()
                    })),
                    None => Some(P(parent.deref().clone()))
                };
            }
        }

        // Create subblocks from a full DescribeState
        let subblocks = self.subblocks.clone().into_iter().map(|block| { block.generate(sp, cx, &self) }).collect();

        // Get a glob import of all items in scope to the module that `describe!` is called from.
        //
        // This glob is `pub use super::*` so that nested `describe!` blocks (which will also contain
        // this glob) will be able to see all the symbols.
        let super_glob = cx.view_use_glob(sp, ast::Public, vec![cx.ident_of("super")]);

        // Generate the new module.
        cx.item_mod(sp, sp, name, vec![], vec![super_glob], subblocks)
    }
}

