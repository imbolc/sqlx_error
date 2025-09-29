[![License](https://img.shields.io/crates/l/sqlx-error.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/sqlx-error.svg)](https://crates.io/crates/sqlx-error)
[![Docs.rs](https://docs.rs/sqlx-error/badge.svg)](https://docs.rs/sqlx-error)

# sqlx-error

A wrapper around `sqlx::Error` to provide error path and additional context.

## Usage

```rust
use sqlx_error::{sqlx_error, SqlxError};

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error(transparent)]
    Sqlx(#[from] SqlxError),
}

/// If you have a single sqlx query per function, the function path by itself could provide
/// enough context
fn foo() -> Result<(), MyError> {
    Err(sqlx::Error::RowNotFound).map_err(sqlx_error!())?;
    Ok(())
}

/// Or you can add more context
fn bar() -> Result<(), MyError> {
    Err(sqlx::Error::RowNotFound).map_err(sqlx_error!("more context"))?;
    Ok(())
}

fn main() {
    assert_eq!(
        foo().unwrap_err().to_string(),
        "sqlx: rust_out::foo at src/lib.rs:15"
    );
    assert_eq!(
        bar().unwrap_err().to_string(),
        "sqlx: more context in rust_out::bar at src/lib.rs:21"
    );
}
```

## Contributing

Please run [.pre-commit.sh] before sending a PR, it will check everything.

This project is licensed under the [MIT license][license].

[.pre-commit.sh]: https://github.com/imbolc/sqlx_error/blob/main/.pre-commit.sh
[license]: https://github.com/imbolc/sqlx_error/blob/main/LICENSE
