use syntax::ptr::P;
use syntax::ast;

/// A test as a description and associated block.
#[derive(Clone)]
pub struct Test {
    pub description: String,
    pub block: P<ast::Block>,
    pub test_config: TestConfig
}

#[derive(Clone)]
pub struct TestConfig {
    pub ignored: bool,
    pub failing: bool,
    pub failing_msg: Option<&'static str>,
}

impl TestConfig {

    pub fn failing_test(failing_msg: Option<&'static str>) -> TestConfig {
        TestConfig {
            failing: true,
            ignored: false,
            failing_msg: failing_msg,
        }
    }

    pub fn ignored_test() -> TestConfig {
        TestConfig {
            failing: false,
            ignored: true,
            failing_msg: None,
        }
    }

    pub fn test() -> TestConfig {
        TestConfig {
            failing: false,
            ignored: false,
            failing_msg: None,
        }
    }
}
