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

use syntax::ptr::P;
use syntax::ast;
use syntax::parse::token;

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
    pub failing_msg: Option<(token::InternedString, ast::StrStyle)>,
}

impl TestConfig {

    pub fn failing_test(failing_msg: Option<(token::InternedString, ast::StrStyle)>) -> TestConfig {
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
