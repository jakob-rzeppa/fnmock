//! Argument matching for the spy of a single function.
//!
//! This is a fnmock internal. You should not interact with it directly.

use std::fmt::Display;

/// Decides whether a call's arguments satisfy an expectation.
///
/// One implementor is generated per spied function: a struct holding one boxed
/// [`Predicate`](predicates::Predicate) per parameter, whose [`Matcher::matches`] evaluates
/// each predicate against the matching argument.
///
/// # Why `Params` is a generic associated type
///
/// A spy observes its arguments by reference, so the parameter tuple of a function like
/// `fn get_user(id: String, uuid: &str)` is `(&String, &str)` — it borrows, and it borrows for
/// a different lifetime on every call. Erasing that tuple behind [`Any`](std::any::Any) is
/// therefore impossible, since `Any` is bounded `'static`.
///
/// Making `Params` a generic associated type instead lets [`SpyStore`](crate::spy_store::SpyStore)
/// name the parameter type without pinning down its lifetime: the lifetime is chosen at each
/// call site, so borrowed arguments never have to outlive the call.
pub trait Matcher: Clone + Display + 'static {
    /// The spied function's parameters, borrowed for the duration of a single call.
    ///
    /// For `fn get_user(id: String, uuid: &str)` this is `(&'a String, &'a str)`.
    type Params<'a>;

    /// Whether `params` satisfies this matcher.
    fn matches(&self, params: &Self::Params<'_>) -> bool;
}
