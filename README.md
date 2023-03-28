# extest

Extra utilities for writing/running the tests.

# Usage

## Groups

```rust
use extest::group;

// Important to keep #[group(..)] macro above the test
// Otherwise it will fail to compile due to alone #[test] macro in the result.
//
// NOTE: Q: Can we replace test absence with empty test body?
#[group(one_eq_one, two_eq_two)]
#[test]
fn test_arithmetic() {
    assert_eq!(1 + 0, 1);
    assert_eq!(1 + 1, 2);
}
```

Run tests:
```compile_fail
$ # test with the `one_eq_one` group will not run
$ GROUP_DISABLE=one_eq_one cargo test
$ # Only enabled tests will be run
$ GROUP_RUN_STRATEGY=only_enabled cargo test # no tests will run
$ # Tests with all mentioned groups will be run
$ GROUP_RUN_STRATEGY=only_enabled GROUP_ENABLE="one_eq_one two_eq_two" cargo test # test_arithmetic... ok
$ GROUP_RUN_STRATEGY=only_enabled GROUP_ENABLE="one_eq_one" cargo test # no tests will run
```

See pre-RFC for official support of this: https://internals.rust-lang.org/t/pre-rfc-test-groups/18591.
Attention required.