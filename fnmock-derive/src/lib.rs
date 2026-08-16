//! Proc-macro implementation behind [`fnmock`](https://docs.rs/fnmock).
//!
//! Everything here except the [`macro@fakeable`] and [`macro@spyable`] attributes is a fnmock
//! internal. Depend on the `fnmock` crate rather than on this one; it re-exports the attributes as
//! `fnmock::fakeable` and `fnmock::spyable`.
//!
//! See the [README](https://github.com/jakob-rzeppa/fnmock/blob/master/README.md) for installation,
//! a walkthrough and the current limitations.

use crate::{fakeable::handle_fakeable, spyable::handle_spyable};

mod expandable;
mod expanded;
mod fakeable;
mod item_info;
mod scheme;
mod spyable;
mod strategy;

/// Make a function or an inherent impl block fakeable in tests.
///
/// Applied to a function, the attribute leaves the original body in place and injects a
/// `#[cfg(test)]`-gated lookup at the top of it: if a fake is installed for this function on the
/// current thread, the fake runs instead of the body. It also generates an accessor named after
/// the function — `fetch_user` gets `fetch_user_fake()` — which tests use to control the fake:
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
///
/// Applied to an inherent impl block, every method in it becomes fakeable and the accessors are
/// generated as associated functions on the same type.
///
/// Fakes are stored per thread, and the test harness gives each `#[test]` its own thread, so tests
/// cannot leak into one another and no reset step is needed. The flip side is that a fake is only
/// visible on the thread that installed it.
///
/// Because the injected lookup is `#[cfg(test)]`-gated, release builds keep the original function
/// body and compile no fake machinery at all.
///
/// # Errors
///
/// The macro can return a compile error if something went wrong or an unsupported construct was used.
/// See `CONSTRAINTS.md` for the full list of unsupported constructs.
#[proc_macro_attribute]
pub fn fakeable(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // This is the only place proc_macro::TokenStream should appear: the actual proc-macro ABI
    // boundary requires it, but proc_macro::TokenStream cannot be constructed or parsed outside
    // a live macro expansion (it panics), which makes anything using it untestable. Converting to
    // proc_macro2::TokenStream here lets the rest of the crate be tested with ordinary unit tests.
    let res = handle_fakeable(attr.into(), item.into());

    match res {
        Ok(expanded) => expanded.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Record a free function's calls in tests, and assert on them.
///
/// Where [`macro@fakeable`] replaces a function's body, a spy leaves it alone: the attribute
/// injects a `#[cfg(test)]`-gated statement at the top of the body that hands the call's arguments
/// to the spy on the way past, and then the real implementation runs. It also generates an
/// accessor named after the function — `get_user` gets `get_user_spy()` — which tests set
/// expectations on:
///
/// | Method | Behaviour |
/// | --- | --- |
/// | `expect(pred, ..)` | Expect calls whose arguments satisfy one predicate per parameter. |
/// | `expectf(closure)` | Expect calls whose arguments satisfy one closure over all of them. |
/// | `expect_times(n)` / `expect_once()` / `expect_never()` | Expect this many calls, whatever their arguments. |
/// | `assert()` | Assert every expectation set on this spy is fulfilled. |
///
/// `expect` and `expectf` hand back a handle that refines the expectation by chaining —
/// `times(2)`, `once()`, `never()`, `in_sequence(&mut seq)`. See
/// [EXPECTATIONS.md](https://github.com/jakob-rzeppa/fnmock/blob/master/docs/EXPECTATIONS.md).
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
///
/// Arguments are matched by shared reference, so a `String` parameter is matched by a
/// `Predicate<String>` and a `&str` one by a `Predicate<str>`; nothing is cloned or moved out of
/// the call.
///
/// Spies are stored per thread, and the test harness gives each `#[test]` its own thread, so tests
/// cannot leak into one another. Because the injected statement is `#[cfg(test)]`-gated, release
/// builds keep the original function body and compile no spy machinery at all.
///
/// # Errors
///
/// Only free functions can be spied on so far: impl blocks, generic type and const parameters, and
/// parameters that are not plain identifiers are all rejected with a compile error. See
/// `CONSTRAINTS.md` for the full list.
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
