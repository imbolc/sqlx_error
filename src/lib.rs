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
//! assert_eq!(foo().unwrap_err().to_string(), "sqlx rust_out::foo, src/lib.rs:15:43");
//! assert_eq!(bar().unwrap_err().to_string(), "sqlx rust_out::bar, src/lib.rs:21:43, more context");
//! # }
//! ```

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

use ::std::{error::Error, fmt, option::Option};

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
        |e| $crate::SqlxError::new(e, code_path::with_loc!())
    };
    ($desc:expr) => {
        |e| $crate::SqlxError::new(e, format!("{}, {}", code_path::with_loc!(), $desc))
    };
}

impl SqlxError {
    /// Creates an `SqlxError` instance
    pub fn new(err: sqlx_core::error::Error, msg: String) -> Self {
        Self(err, msg)
    }
}

impl fmt::Display for SqlxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sqlx {}", self.1)
    }
}

impl Error for SqlxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Option::Some(&self.0)
    }
}
