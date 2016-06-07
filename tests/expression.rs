// Copyright 2016 Taylor Cramer
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

#![feature(plugin)]
#![plugin(stainless)]

describe! expression_at_end_of_block {
    before_each {
        let x = 5;
        let y = 6;
        let mut z = 0;
        for _ in 0..5 {
            z += 1;
        }
    }

    it "should execute expressions at ends of test blocks as statements" {
        assert_eq!(x + y, 11);
        assert_eq!(z, 5);
        for _ in 0..5 {
            z += 1;
        }
    }

    after_each {
        assert_eq!(x, 5);
        assert_eq!(y, 6);
        assert_eq!(z, 10);
        for _ in 0..5 {
            // Purposefully empty-- tests that after_each can end with loop
        }
    }
}
