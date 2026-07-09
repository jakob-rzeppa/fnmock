//! Associated type bounds are not supported. If possible you should use associated type equality instead.
//!
//! This is not supported:
//!
//! ```rust
//! #[fnmock::fakeable]
//! fn associated_type_bounds<I>(value: I) -> Vec<String> where I: Iterator + 'static, I::Item: Display {
//!    value.map(|item| item.to_string()).collect()
//! }
//!```
