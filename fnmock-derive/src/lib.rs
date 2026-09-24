//! Proc-macro implementation behind [`fnmock`](https://docs.rs/fnmock).
//!
//! Everything here except the [`macro@mockable`] attribute and its two single-half variants,
//! [`macro@fakeable`] and [`macro@spyable`], is a fnmock internal. Depend on the `fnmock` crate
//! rather than on this one. It re-exports the attributes as `fnmock::mockable`, `fnmock::fakeable`
//! and `fnmock::spyable`.
//!
//! See the [README](https://github.com/jakob-rzeppa/fnmock/blob/master/README.md) for installation,
//! a walkthrough and the current limitations.

use crate::{fakeable::handle_fakeable, mockable::handle_mockable, spyable::handle_spyable};

mod entry;
mod expandable;
mod expanded;
mod fakeable;
mod item_info;
mod mockable;
mod scheme;
mod spyable;
mod strategy;

/// Mock a free function or the methods of an inherent impl block in tests.
///
/// The attribute leaves the original body in place and injects a `#[cfg(test)]`-gated block at the
/// top of it. The block records the call. Then, if the test has installed a replacement closure on
/// the current thread, the closure's result is returned instead of running the body. The attribute
/// also generates an accessor named after the function (`get_user` gets `get_user_mock()`), which
/// tests use to control the mock:
///
/// | Method | Behaviour |
/// | --- | --- |
/// | `setup(closure)` | Replace the body with a closure of the same signature. Calling it again replaces the previous one. |
/// | `is_set()` | Whether a replacement closure is installed. |
/// | `expect(pred, ..)` | Expect calls whose arguments satisfy one predicate per parameter. |
/// | `expectf(closure)` | Expect calls whose arguments satisfy one closure over all of them. |
/// | `expect_times(n)` / `expect_once()` / `expect_never()` | Expect this many calls, whatever their arguments. |
/// | `assert()` | Assert every expectation set on this mock is fulfilled. |
/// | `clear()` | Reset the whole mock: remove the closure, and drop every expectation and recorded call. |
///
/// ```ignore
/// #[fnmock::mockable]
/// fn fetch_user_name(id: u32) -> String {
///     // real database call
/// }
///
/// #[test]
/// fn test_greeting() {
///     let mock = fetch_user_name_mock();
///     mock.setup(|_| "Test".into());
///     mock.expect(fnmock::predicate::eq(1)).once();
///
///     assert_eq!(greet(1), "Hello, Test");
///
///     mock.assert();
/// }
/// ```
///
/// Every call is recorded, including calls the closure answers, so a canned return value and an
/// assertion on the call work together. A call the closure answers never runs the real body, so the
/// real body's side effects don't happen. Arguments are matched by shared reference: a `String`
/// parameter is matched by a `Predicate<String>` and a `&str` one by a `Predicate<str>`.
///
/// `expect` and `expectf` return a handle that refines the expectation by chaining: `times(2)`,
/// `once()`, `never()`, `describe(..)`, `in_sequence(&seq)`. See
/// [FEATURES.md](https://github.com/jakob-rzeppa/fnmock/blob/master/docs/FEATURES.md).
///
/// Applied to an inherent impl block, every method in it gets its own mock, and the accessors are
/// generated as associated functions on the same type (`Type::method_mock()`). The `setup` closure
/// receives the receiver as its first argument. Expectations never match on the receiver.
///
/// Mocks are stored per thread, and the test harness gives each `#[test]` its own thread, so tests
/// can't leak into one another and no reset step is needed. The flip side is that a mock is only
/// visible on the thread that set it up. Because the injected block is `#[cfg(test)]`-gated,
/// release builds keep the original body and compile no mock code at all.
#[proc_macro_attribute]
pub fn mockable(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // These attribute functions are the only place proc_macro::TokenStream should appear: the
    // actual proc-macro ABI boundary requires it, but proc_macro::TokenStream cannot be constructed
    // or parsed outside a live macro expansion (it panics), which makes anything using it
    // untestable. Converting to proc_macro2::TokenStream here lets the rest of the crate be tested
    // with ordinary unit tests.
    let res = handle_mockable(attr.into(), item.into());

    match res {
        Ok(expanded) => expanded.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Replace a free function's or impl method's body in tests, without recording calls.
///
/// This is the replacing half of [`macro@mockable`] on its own. It generates `<fn_name>_fake()`:
///
/// | Method | Behaviour |
/// | --- | --- |
/// | `setup(closure)` | Install a fake, replacing any previous one. |
/// | `clear()` | Remove the fake; calls run the real body again. |
/// | `is_set()` | Whether a fake is currently installed. |
///
/// ```ignore
/// #[fnmock::fakeable]
/// fn fetch_user(id: u32) -> User {
///     // real database call
/// }
///
/// #[test]
/// fn returns_the_faked_user() {
///     fetch_user_fake().setup(|id| User { id, name: "Test".into() });
///
///     assert_eq!(fetch_user(1).name, "Test");
/// }
/// ```
#[proc_macro_attribute]
pub fn fakeable(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let res = handle_fakeable(attr.into(), item.into());

    match res {
        Ok(expanded) => expanded.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Record a free function's or impl method's calls in tests and assert on them. The real body
/// always runs.
///
/// This is the recording half of [`macro@mockable`] on its own. It generates `<fn_name>_spy()`
/// with the mock's expectation methods, `expect`, `expectf`, `expect_times`, `expect_once`,
/// `expect_never` and `assert`, plus a `clear()` that drops the expectations and call history. It
/// has no `setup` or `is_set`.
///
/// ```ignore
/// #[fnmock::spyable]
/// fn get_user(id: String, uuid: &str) -> String {
///     // real database call
/// }
///
/// #[test]
/// fn asks_for_the_user_once() {
///     let spy = get_user_spy();
///     spy.expect(predicate::eq("a".to_string()), predicate::always()).once();
///
///     get_user("a".to_string(), "uuid");
///
///     spy.assert();
/// }
/// ```
#[proc_macro_attribute]
pub fn spyable(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let res = handle_spyable(attr.into(), item.into());

    match res {
        Ok(expanded) => expanded.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
