# Stainless [![Build Status](https://travis-ci.org/reem/stainless.svg?branch=master)](https://travis-ci.org/reem/stainless)

> Stainless is a lightweight, flexible, unopinionated testing framework.

## Example

```ignore
describe! stainless {
    before_each {
        // Start up a test.
        let mut stainless = true;
    }

    it "makes organizing tests easy" {
        // Do the test.
        assert!(stainless);
    }

    after_each {
        // End the test.
        stainless = false;
    }

    bench "something simple" (bencher) {
        bencher.iter(|| 2u * 2)
    }

    describe! nesting {
        it "makes it simple to categorize tests" {
            // It even generates submodules!
            assert_eq!(2u, 2u);
        }
    }
}
```

Expands to (roughly):

```rust
mod stainless {
    #[test]
    fn makes_organizing_tests_easy() {
        let mut stainless = true;
        assert!(stainless);
        stainless = false;
    }

    #[bench]
    fn something_simple(bencher: &mut test::Bencher) {
        bencher.iter(|| 2u * 2)
    }

    mod nesting {
        #[test]
        fn makes_it_simple_to_categorize_tests() {
            assert_eq!(2u, 2u);
        }
    }
}
```

## Overview

Stainless exports the `describe!` syntax extension, which allows
you to quickly generate complex testing hierarchies and reduce
boilerplate through `before_each` and `after_each`.

Stainless currently supports 4 types of subblocks:
 - `before_each` and `after_each`,
 - `it` and `failing`
 - `bench`
 - nested `describe!`

`before_each` and `after_each` allow you to group common initialization
and teardown for a group of tests into a single block, shortening your
tests.

`it` generates tests which use `before_each` and `after_each`. `failing`
does the same, except the generated tests are marked with `#[should_fail]`.

`bench` allows you to generate benchmarks in the same fashion, though
`before_each` and `after_each` blocks do not affect `bench` blocks.

Nested `describe!` blocks allow you to better organize your tests into
small units and gives you granular control over where `before_each` and
`after_each` apply.

Together, these 4 types of subblocks give you more flexibility and control
than the built in testing infrastructure.

## License

MIT

