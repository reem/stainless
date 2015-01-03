use syntax::ast;
use syntax::ptr::P;

/// A benchmark, represented as a description, an associated block,
/// and an ident for the name of the Bencher argument.
#[derive(Clone)]
pub struct Bench {
    pub bench: P<ast::Ident>,
    pub description: String,
    pub block: P<ast::Block>
}

