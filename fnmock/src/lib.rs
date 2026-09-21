//! A Rust mocking framework for standalone functions and methods in an impl block.
//!
//! fnmock lets you replace or observe a function's behaviour in tests without introducing a trait
//! or dependency injection wiring. You annotate the function where it already lives, and the test
//! either controls what it returns ([`fakeable`]) or asserts on how it was called ([`spyable`]).
//!
//! # Fakes
//!
//! [`fakeable`] injects a lookup at the top of the function: if the test installed a fake, the
//! fake runs instead of the body.
//!
//! ```ignore
//! #[fnmock::fakeable]
//! fn fetch_user_name(id: u32) -> String {
//!     todo!()
//! }
//!
//! fn greet(id: u32) -> String {
//!     format!("Hello, {}", fetch_user_name(id))
//! }
//!
//! #[test]
//! fn test_greeting() {
//!     fetch_user_name_fake().setup(|_| "Test".into());
//!
//!     assert_eq!(greet(1), "Hello, Test");
//! }
//! ```
//!
//! # Spies
//!
//! [`spyable`] works the other way round: the real body still runs, and the test asserts on the
//! arguments it was called with.
//!
//! ```ignore
//! #[fnmock::spyable]
//! fn fetch_user_name(id: u32) -> String {
//!     format!("user {id}")
//! }
//!
//! #[test]
//! fn test_fetch_user_name() {
//!     let spy = fetch_user_name_spy();
//!     spy.expect(fnmock::predicate::eq(1)).once();
//!
//!     assert_eq!(fetch_user_name(1), "user 1"); // the real body still runs
//!
//!     spy.assert();
//! }
//! ```
//!
//! `expect` and `expectf` hand back an [`ExpectationHandle`], which refines the expectation by
//! chaining: `times` (taking anything that converts into a [`CallRange`] — a count or a range),
//! `once`, `never`, `describe` and `in_sequence`, where a [`Sequence`] orders expectations across
//! several spies.
//!
//! # Crate layout
//!
//! [`fakeable`] and [`spyable`] are the whole API you apply to production code. The types a test
//! touches are [`ExpectationHandle`], [`CallRange`], [`Sequence`] and the re-exported
//! [`predicate`] builders. Every other module — the fake and spy stores, matchers, expectations —
//! only exists because the code the macros expand to has to name it; they are fnmock internals and
//! you should not interact with them directly.
//!
//! Fakes and spies are kept per thread and their lookup is `#[cfg(test)]`-gated, so each `#[test]`
//! gets its own isolated state and release builds compile no fake or spy machinery at all. The
//! flip side is that both can only be set up from a `#[cfg(test)]` unit test inside the crate that
//! defines the annotated item.
//!
//! See the [README](https://github.com/jakob-rzeppa/fnmock/blob/master/README.md) for installation,
//! a walkthrough and the current limitations.

#[doc(hidden)]
pub mod call_range;
#[doc(hidden)]
pub mod expectation;
#[doc(hidden)]
pub mod expectation_handle;
#[doc(hidden)]
pub mod fake_store;
#[doc(hidden)]
pub mod generic_fake_store;
#[doc(hidden)]
pub mod generic_spy_store;
#[doc(hidden)]
pub mod matcher;
#[doc(hidden)]
pub mod sequence;
#[doc(hidden)]
pub mod spy_store;

// Re-export the public API to make doc-comments visible.
pub use call_range::CallRange;
pub use expectation_handle::ExpectationHandle;
pub use sequence::Sequence;

pub use predicates::{
    boolean::PredicateBooleanExt,
    prelude::{Predicate, PredicateBoxExt, PredicateFileContentExt, PredicateStrExt, predicate},
};

/// Re-export the derive macro so that users of the library can just use `fnmock::fakeable` instead of having to depend on `fnmock-derive` directly.
pub use fnmock_derive::*;
