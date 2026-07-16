//! Reference patterns are not supported for fakes.
//!
//! ```
//! #[fnmock::fakeable]
//! fn reference_patterns((ref left, right): (i32, i32)) -> i32 {
//!     left + right
//! }
//! ```
//!
//! The signature of the fake function will need a owned value, not a reference, and we cannot obtain a value from a reference in the general case.
//! Therefore, we do not support `ref` patterns for fake call values.
