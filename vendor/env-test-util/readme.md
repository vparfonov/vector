# Env test util

[![Crates.io](https://img.shields.io/crates/d/env-test-util)](https://crates.io/crates/env-test-util)
![Crates.io](https://img.shields.io/crates/v/env-test-util)

[![Build Status](https://travis-ci.com/jdrouet/env-test-util.svg?branch=master)](https://travis-ci.com/jdrouet/env-test-util)
[![codecov](https://codecov.io/gh/jdrouet/env-test-util/branch/master/graph/badge.svg?token=C5ABZY8YRQ)](https://codecov.io/gh/jdrouet/env-test-util)

Just a simple tool to manipulate environment variable in tests.

## Usage

When initialising the variable manager with `new`, the actual content will be removed and stored
in `initial_value`. You can then set a temporary value using the method `with`. The environment
variable will then be reset to it's initial value when it will be dropped.

## Examples

```rust
use env_test_util::TempEnvVar;

std::env::set_var("MY_VARIABLE", "ORIGINAL"); // set the variable to "ORIGINAL"
let variable = TempEnvVar::new("MY_VARIABLE"); // read the variable and stores it
assert_eq!(std::env::var("MY_VARIABLE").ok(), None);
let variable = variable.with("NEW_CONTENT"); // set the environment variable with a new content
assert_eq!(std::env::var("MY_VARIABLE").ok(), Some("NEW_CONTENT".into()));
drop(variable);
assert_eq!(std::env::var("MY_VARIABLE").ok(), Some("ORIGINAL".into()));
```

Don't forget to assign the variable in your tests, otherwise the `drop` function will be called right away

```rust
use env_test_util::TempEnvVar;

std::env::set_var("MY_VARIABLE", "ORIGINAL"); // set the variable to "ORIGINAL"
TempEnvVar::new("MY_VARIABLE").with("SOMETHING_ELSE"); // read the variable and stores it
assert_eq!(std::env::var("MY_VARIABLE").ok(), Some("ORIGINAL".into()));
let _variable = TempEnvVar::new("MY_VARIABLE").with("SOMETHING_ELSE"); // Instead, store it in a variable
assert_eq!(std::env::var("MY_VARIABLE").ok(), Some("SOMETHING_ELSE".into()));
```

## Real life example

```rust
#[test]
fn testing_conntection_database() {
    let _original_url = TempEnvVar::new("DATABASE_URL").with("postgres://wrong-url");
    let connection = Database::connect(); // something that reads the environment variable
    assert!(connection.is_err());
}
```
