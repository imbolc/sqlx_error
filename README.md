[![License](https://img.shields.io/crates/l/sqlx-error.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/sqlx-error.svg)](https://crates.io/crates/sqlx-error)
[![Docs.rs](https://docs.rs/sqlx-error/badge.svg)](https://docs.rs/sqlx-error)

<!-- cargo-sync-readme start -->

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

assert_eq!(foo().unwrap_err().to_string(), "sqlx rust_out::foo");
assert_eq!(bar().unwrap_err().to_string(), "sqlx rust_out::bar more context");
```

<!-- cargo-sync-readme end -->

## Contributing

We appreciate all kinds of contributions, thank you!


### Note on README

Most of the readme is automatically copied from the crate documentation by [cargo-sync-readme][].
This way the readme is always in sync with the docs and examples are tested.

So if you find a part of the readme you'd like to change between `<!-- cargo-sync-readme start -->`
and `<!-- cargo-sync-readme end -->` markers, don't edit `README.md` directly, but rather change
the documentation on top of `src/lib.rs` and then synchronize the readme with:
```bash
cargo sync-readme
```
(make sure the cargo command is installed):
```bash
cargo install cargo-sync-readme
```

If you have [rusty-hook] installed the changes will apply automatically on commit.


## License

This project is licensed under the [MIT license](LICENSE).

[cargo-sync-readme]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
