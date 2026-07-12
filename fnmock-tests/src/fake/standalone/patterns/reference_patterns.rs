//! Reference patterns are supported for fakes.
//!
//! ```
//! #[fnmock::fakeable]
//! fn reference_patterns((ref left, right): (i32, i32)) -> i32 {
//!     left + right
//! }
//! ```
//!
//! This is because we can ignore the `ref` keyword in the pattern and just get the identifier name for the fake call value.
//! The signature of the fake function will need a value, not a reference, and we cannot obtain a value from a reference in the general case.
//! Therefore, we do not support `ref` patterns for fake call values.
