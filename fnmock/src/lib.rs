//! A Rust mocking framework for standalone functions and methods in an impl block.
//!
//! fnmock lets you replace a function's behaviour in tests without introducing a trait or
//! dependency injection wiring. You annotate the function where it already lives with
//! [`fakeable`], and the test controls what it returns:
//!
//! ```ignore
//! #[fnmock::fakeable]
//! fn fetch_user(id: u32) -> User {
//!     // real database call
//! }
//!
//! #[test]
//! fn test_greeting() {
//!     fetch_user_fake().setup(|id| User { id, name: "Test".into() });
//!
//!     assert_eq!(greet(1), "Hello, Test");
//! }
//! ```
//!
//! # Crate layout
//!
//! [`fakeable`] is the entire public API for fakes. The [`fake_store`] and [`generic_fake_store`] modules
//! only exist because the code the macro expands to has to name them; they are fnmock internals
//! and you should not interact with them directly.
//!
//! See the [README](https://github.com/jakob-rzeppa/fnmock/blob/master/README.md) for installation,
//! a walkthrough and the current limitations.

pub mod fake_store;
pub mod generic_fake_store;

/// Re-export the derive macro so that users of the library can just use `fnmock::fakeable` instead of having to depend on `fnmock-derive` directly.
pub use fnmock_derive::*;
