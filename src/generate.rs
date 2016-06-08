// Copyright 2014-2016 The Stainless Developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::ops::Deref;

use syntax::{ast, abi, codemap};
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

fn block_to_stmts(block: &ast::Block) -> Vec<ast::Stmt> {
    use syntax::codemap::Spanned;

    let block = block.clone();

    let id = block.id;
    let stmts = block.stmts;
    let expr = block.expr;

    stmts
        .iter()
        .chain(expr.map(|expr| {
            let span = expr.span;
            Spanned {
                node: ast::StmtKind::Expr(expr, id),
                span: span,
            }
        }).iter())
        .cloned()
        .collect()
}

impl<'a> Generate<&'a DescribeState> for Test {
    fn generate(self, sp: codemap::Span, cx: &mut base::ExtCtxt, state: &'a DescribeState) -> P<ast::Item> {
        let Test { description, block, test_config } = self;

        // Create the #[test] attribute.
        let test_attribute = cx.attribute(sp, cx.meta_word(sp, token::InternedString::new("test")));

        // Create the #[should_panic] attribute.
        let should_panic = cx.attribute(sp, cx.meta_word(sp, token::InternedString::new("should_panic")));

        // Create the #[ignore] attribute.
        let ignore = cx.attribute(sp, cx.meta_word(sp, token::InternedString::new("ignore")));

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
                    stmts: block_to_stmts(&before).iter()
                            .chain(block_to_stmts(&block).iter())
                            .cloned().collect(),
                    ..block.deref().clone()
                })
            },

            (&None, &Some(ref after)) => {
                P(ast::Block {
                    stmts: block_to_stmts(&block).iter()
                            .chain(block_to_stmts(&after).iter())
                            .cloned().collect(),
                    ..block.deref().clone()
                })
            },

            (&Some(ref before), &Some(ref after)) => {
                P(ast::Block {
                    stmts: block_to_stmts(&before).iter()
                            .chain(block_to_stmts(&block).iter())
                            .chain(block_to_stmts(&after).iter())
                            .cloned().collect(),
                    ..block.deref().clone()
                })
            }
        };

        // Constructing attributes:
        // #[test] - no way without it
        // #[allow(non_snake_case_attr)] as description may contain upper case
        // #[should_panic] or #[should_panic(expected = "...")] if specified
        // #[ignore] if specified
        let mut attrs = vec![test_attribute, allow_non_snake_case];
        if test_config.failing {
            match test_config.failing_msg {
                Some(msg) => {
                    // Create #[should_panic(expected = "...")] attribute
                    let should_panic_str = token::InternedString::new("should_panic");
                    let expected_str = token::InternedString::new("expected");
                    attrs.push(cx.attribute(sp, cx.meta_list(
                        sp,
                        should_panic_str,
                        vec![cx.meta_name_value(
                            sp,
                            expected_str,
                            ast::LitKind::Str(msg.0, msg.1)
                        )]
                    )));
                },
                None => attrs.push(should_panic)
            };
        }
        if test_config.ignored {
            attrs.push(ignore);
        }

        // Create the final Item that represents the test.
        P(ast::Item {
            // Name it with a snake_case version of the description.
            ident: cx.ident_of(&description.replace(" ", "_")),
            attrs: attrs,
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemKind::Fn(
                // Takes no arguments and returns ()
                P(ast::FnDecl {
                    inputs: vec![],
                    output: ast::FunctionRetTy::Default(sp),
                    variadic: false
                }),
                // All the usual types.
                ast::Unsafety::Normal,
                ast::Constness::NotConst,
                abi::Abi::Rust,
                ast::Generics::default(),

                // Add the body of the function.
                test_body
            ),
            // Inherited visibility (not pub)
            vis: ast::Visibility::Inherited,
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
            ident: cx.ident_of(&description.replace(" ", "_")),

            // Add #[test] and possibly #[should_panic]
            attrs: vec![bench_attribute],
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemKind::Fn(
                // Takes one argument of &mut Bencher
                P(ast::FnDecl {
                    inputs: vec![ast::Arg {
                        ty: quote_ty!(cx, &mut ::test::Bencher),
                        pat: quote_pat!(cx, $bench),
                        id: ast::DUMMY_NODE_ID
                    }],
                    output: ast::FunctionRetTy::Default(sp),
                    variadic: false
                }),

                // All the usual types.
                ast::Unsafety::Normal,
                ast::Constness::NotConst,
                abi::Abi::Rust,
                ast::Generics::default(),

                // Add the body of the function.
                block
            ),
            // Inherited visibility (not pub)
            vis: ast::Visibility::Inherited,
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
                        stmts: parent.stmts.iter().chain(&*now.stmts).cloned().collect(),
                        ..now.deref().clone()
                    })),
                    None => Some(P(parent.deref().clone()))
                };
            }

            if let Some(ref parent) = state.after_each {
                self.after_each = match self.after_each {
                    Some(ref now) => Some(P(ast::Block {
                        stmts: now.stmts.iter().chain(&*parent.stmts).cloned().collect(),
                        ..now.deref().clone()
                    })),
                    None => Some(P(parent.deref().clone()))
                };
            }
        }

        // Get a glob import of all items in scope to the module that `describe!` is called from.
        //
        // This glob is `pub use super::*` so that nested `describe!` blocks (which will also contain
        // this glob) will be able to see all the symbols.
        let super_glob = cx.item_use_glob(sp, ast::Visibility::Public, vec![cx.ident_of("super")]);
        let mut items = vec![super_glob];

        // Create subblocks from a full DescribeState
        items.extend(self.subblocks.clone().into_iter().map(|block| {
            block.generate(sp, cx, &self)
        }));

        // Generate the new module.
        cx.item_mod(sp, sp, name, vec![], items)
    }
}
