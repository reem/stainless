use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::ptr::P;
use test::Test;
use bench::Bench;

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

