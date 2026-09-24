//! A Rust mocking framework for standalone functions and methods in an impl block.
//!
//! fnmock lets you mock a plain function in tests without adding a trait or dependency-injection
//! wiring. Annotate the function with [`mockable`] where it already lives. The test can then decide
//! what the function returns and assert on how it was called.
//!
//! ```ignore
//! #[fnmock::mockable]
//! fn fetch_user_name(id: u32) -> String {
//!     // real database call
//! }
//!
//! fn greet(id: u32) -> String {
//!     format!("Hello, {}", fetch_user_name(id))
//! }
//!
//! #[test]
//! fn test_greeting() {
//!     let mock = fetch_user_name_mock();
//!     mock.setup(|_| "Test".into());                  // replace what it returns ...
//!     mock.expect(fnmock::predicate::eq(1)).once();   // ... and expect how it is called
//!
//!     assert_eq!(greet(1), "Hello, Test");
//!
//!     mock.assert();
//! }
//! ```
//!
//! Every call is recorded, including calls the `setup` closure answers. Without `setup`, the real
//! body runs. `clear()` resets the whole mock.
//!
//! `expect` and `expectf` return an [`ExpectationHandle`], which refines the expectation by
//! chaining:
//!
//! - `times`, which takes anything that converts into a [`CallRange`] (a count or a range)
//! - `once`
//! - `never`
//! - `describe`
//! - `in_sequence`, which adds the expectation to a [`Sequence`]
//!
//! A [`Sequence`] orders expectations across several functions.
//!
//! # Only need one half?
//!
//! Two smaller attributes each provide one half of a mock:
//!
//! - [`fakeable`] generates `<fn_name>_fake()` with `setup`, `is_set` and `clear`. It replaces the
//!   body and records nothing. It also accepts destructuring parameters, which a mock can't record.
//! - [`spyable`] generates `<fn_name>_spy()` with the expectation methods only. The real body always
//!   runs. It also accepts `-> impl Trait` and `-> !`, which a mock can't produce.
//!
//! # Crate layout
//!
//! [`mockable`], [`fakeable`] and [`spyable`] are the whole API you apply to production code. The
//! types a test touches are [`ExpectationHandle`], [`CallRange`], [`Sequence`] and the re-exported
//! [`predicate`] builders. Every other module (the stores, matchers and expectations) exists only
//! because the code the macros expand to has to name it. They are fnmock internals, and you
//! shouldn't use them directly.
//!
//! Mocks are kept per thread and their injected code is `#[cfg(test)]`-gated. Each `#[test]`
//! therefore gets its own isolated state, and release builds compile no mock code at all. The flip
//! side is that a mock can only be set up from a `#[cfg(test)]` unit test inside the crate that
//! defines the annotated item.
//!
//! See the [README](https://github.com/jakob-rzeppa/fnmock/blob/master/README.md) for installation,
//! a walkthrough and the current limitations.

#[doc(hidden)]
pub mod common {
    pub mod generic_key;
}
#[doc(hidden)]
pub mod fake {
    pub mod fake_store;
    pub mod generic_fake_store;
}
#[doc(hidden)]
pub mod spy {
    pub mod call_range;
    pub mod expectation;
    pub mod expectation_handle;
    pub mod generic_spy_store;
    pub mod matcher;
    pub mod sequence;
    pub mod spy_store;
}

// Re-export the types a test touches so that users of the library can just use `fnmock::Sequence` and doc comments are visible.
pub use spy::call_range::CallRange;
pub use spy::expectation_handle::ExpectationHandle;
pub use spy::sequence::Sequence;

/// Re-export the predicate builders so that users of the library can just use `fnmock::predicate` instead of having to depend on `predicates` directly.
pub use predicates::{
    boolean::PredicateBooleanExt,
    prelude::{Predicate, PredicateBoxExt, PredicateFileContentExt, PredicateStrExt, predicate},
};

/// Re-export the derive macro.
pub use fnmock_derive::*;
