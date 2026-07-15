//! Ignored parameters like this are not supported.
//!
//! ```
//!  #[fnmock::fakeable]
//! fn ignored(_: String, value: String) -> String {
//!     value
//! }
//! ```
//!
//! You should opt to use `_name` instead of `_` to ignore unused warnings.
//!
//! Ignored parameters are not supported, since we need to pass the parameters to the fakeable function, and we cannot pass `_: String` as a parameter.
