//! Struct destructurting patterns are not supported for fakes.
//!
//! ```
//! #[fnmock::fakeable]
//! fn struct_destructuring(Point { x, y }: Point) -> i32 {
//!     x + y
//! }
//! ```
//!
//! This is because we would need to generate a new struct with the same fields as the original struct, which is not possible in a generic way - for example, if the struct has private fields, we cannot generate a new struct with the same fields. Therefore, we do not support struct destructuring patterns for fakes.
