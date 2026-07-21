//! Proc-macro implementation behind [`fnmock`](https://docs.rs/fnmock).
//!
//! Everything here except the [`macro@fakeable`] attribute is a fnmock internal. Depend on the
//! `fnmock` crate rather than on this one; it re-exports the attribute as `fnmock::fakeable`.
//!
//! See the [README](https://github.com/jakob-rzeppa/fnmock/blob/master/README.md) for installation,
//! a walkthrough and the current limitations.

use crate::fakeable::handle_fakeable;

mod extract;
mod fakeable;
mod module_builder;
mod names;

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
/// See `FEATURES.md` for the full list of unsupported constructs.
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
