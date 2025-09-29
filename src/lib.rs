//! # sqlx-error
//!
//! A wrapper around `sqlx::Error` to provide error path and additional context.
//!
//! ## Usage
//!
//! ```rust
//! use sqlx_error::{sqlx_error, SqlxError};
//!
//! #[derive(Debug, thiserror::Error)]
//! pub enum MyError {
//!     #[error(transparent)]
//!     Sqlx(#[from] SqlxError),
//! }
//!
//! /// If you have a single sqlx query per function, the function path by itself could provide
//! /// enough context
//! fn foo() -> Result<(), MyError> {
//!     Err(sqlx::Error::RowNotFound).map_err(sqlx_error!())?;
//!     Ok(())
//! }
//!
//! /// Or you can add more context
//! fn bar() -> Result<(), MyError> {
//!     Err(sqlx::Error::RowNotFound).map_err(sqlx_error!("more context"))?;
//!     Ok(())
//! }
//!
//! # fn main() {
//! assert_eq!(foo().unwrap_err().to_string(), "sqlx: rust_out::foo at src/lib.rs:15");
//! assert_eq!(bar().unwrap_err().to_string(), "sqlx: more context in rust_out::bar at src/lib.rs:21");
//! # }
//! ```

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

use std::{error::Error, fmt, option::Option};

/// Sqlx error wrapper to hold additional info
#[derive(Debug)]
pub struct SqlxError(::sqlx_core::error::Error, String);

/// A `Result` based on `SqlxError`
pub type SqlxResult<T> = Result<T, SqlxError>;

/// The macro adds error path and optional description to`sqlx::Error`.
///
/// If you have a single sqlx query per function and the function path by itself provides enough
/// context you can just use `sqlx_error!()`. If it's not enough you can provide an additional
/// message with `sqlx_error!("more context")`.
#[macro_export]
macro_rules! sqlx_error {
    () => {
        |e| $crate::SqlxError::new(e, $crate::__private::code_path::code_path!().into())
    };
    ($desc:expr) => {
        |e| {
            $crate::SqlxError::new(
                e,
                format!(
                    "{} in {}",
                    $desc,
                    $crate::__private::code_path::code_path!()
                ),
            )
        }
    };
}

#[doc(hidden)]
pub mod __private {
    pub use code_path;
}

impl SqlxError {
    /// Creates an `SqlxError` instance
    pub fn new(err: sqlx_core::error::Error, msg: String) -> Self {
        Self(err, msg)
    }
}

impl fmt::Display for SqlxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sqlx: {}", self.1)
    }
}

impl Error for SqlxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Option::Some(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, thiserror::Error)]
    pub enum MyError {
        #[error(transparent)]
        Sqlx(#[from] SqlxError),
    }

    fn bare() -> Result<(), MyError> {
        Err(sqlx::Error::RowNotFound).map_err(sqlx_error!())?;
        Ok(())
    }

    fn with_context() -> Result<(), MyError> {
        Err(sqlx::Error::RowNotFound).map_err(sqlx_error!("my context"))?;
        Ok(())
    }

    #[test]
    fn works() {
        assert!(bare()
            .unwrap_err()
            .to_string()
            .starts_with("sqlx: sqlx_error::tests::bare at src/lib.rs:"));

        assert!(with_context()
            .unwrap_err()
            .to_string()
            .starts_with("sqlx: my context in sqlx_error::tests::with_context at src/lib.rs:"));
    }
}
