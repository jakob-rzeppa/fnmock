//! The information needed to generate a spy module.

/// Information needed to generate a spy module (the matcher + `thread_local` store + interface
/// struct).
///
/// Every field that would otherwise require iterating over the spied function's parameters (one
/// entry per parameter, in declaration order) is instead pre-rendered as a single
/// [`proc_macro2::TokenStream`] holding exactly the code that belongs at that point in the
/// generated module. `generate_spy_module_code` only substitutes these fields into a fixed
/// template; it does not iterate over parameters, join strings, or otherwise assemble per-field
/// code itself.
///
/// Every example below is what the fields would hold for:
///
/// ```ignore
/// fn get_user(id: String, uuid: &str) -> String { .. }
/// ```
#[derive(Clone)]
pub struct SpyModuleInfo {
    /// The name of the generated module itself.
    ///
    /// Example: `get_user_spy_module`.
    pub module_name: syn::Ident,

    /// The name of the `thread_local` static holding the store.
    ///
    /// Example: `SPY`.
    pub store_name: syn::Ident,

    /// How the spied function is referred to in panic messages, e.g. `"UserService get_user"`.
    /// Used both as the `SpyStore`'s own name and as the `function_name` passed to
    /// `ExpectationHandle::new`.
    ///
    /// Example: `"get_user".to_string()`.
    pub display_name: String,

    /// The name of the generated matcher enum.
    ///
    /// Example: `GetUserMatcher`.
    pub matcher_name: syn::Ident,

    /// The param identifiers
    pub param_idents: Vec<syn::Ident>,

    /// The types of the params with references (if used) stripped.
    ///
    /// (&str, String) becomes (str, String)
    pub param_types_unreferenced: Vec<syn::Type>,

    /// The name of the generated interface struct carrying `expect`/`expectf`/`expect_times`/
    /// `assert`/etc.
    ///
    /// Example: `GetUserSpyInterface`.
    pub interface_struct_name: syn::Ident,
}
