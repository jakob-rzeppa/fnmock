//! The information needed to generate a spy module.

use crate::{
    old::extract::{
        function::info::FunctionInfo,
        params::{extract_param_idents, strip_reference},
    },
    old::names::{
        NameType, build_interface_struct_name, build_matcher_name, build_module_name,
        build_store_name,
    },
};

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

    pub visibility: syn::Visibility,

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

impl TryFrom<&FunctionInfo> for SpyModuleInfo {
    type Error = syn::Error;

    fn try_from(function_info: &FunctionInfo) -> Result<Self, Self::Error> {
        Ok(SpyModuleInfo {
            module_name: build_module_name(&function_info.name, NameType::Spy),
            store_name: build_store_name(&function_info.name, NameType::Spy),
            visibility: function_info.visibility.clone(),
            display_name: function_info.name.to_string(),
            matcher_name: build_matcher_name(&function_info.name, NameType::Spy),
            param_idents: extract_param_idents(&function_info.param_pats, NameType::Spy)?,
            param_types_unreferenced: function_info
                .param_types
                .iter()
                .map(strip_reference)
                .collect(),
            interface_struct_name: build_interface_struct_name(&function_info.name, NameType::Spy),
        })
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;
    use crate::old::extract::function::extract_function_info;

    fn function_info(item_fn: syn::ItemFn) -> FunctionInfo {
        extract_function_info(&item_fn, NameType::Spy).expect("valid standalone function")
    }

    fn render<T: ToTokens>(items: &[T]) -> Vec<String> {
        items
            .iter()
            .map(|item| item.to_token_stream().to_string())
            .collect()
    }

    #[test]
    fn test_try_from_function_info_names_every_generated_item_after_the_function() {
        let info = SpyModuleInfo::try_from(&function_info(syn::parse_quote! {
            fn get_user(id: String, uuid: &str) -> String {
                todo!()
            }
        }))
        .expect("conversion should succeed for a standalone function");

        assert_eq!(info.module_name.to_string(), "get_user_spy_module");
        assert_eq!(info.store_name.to_string(), "GET_USER_SPY_STORE");
        assert_eq!(info.display_name, "get_user");
        assert_eq!(info.matcher_name.to_string(), "GetUserMatcher");
        assert_eq!(
            info.interface_struct_name.to_string(),
            "GetUserSpyInterface"
        );
    }

    /// A spy observes its arguments by shared reference, so a parameter the user already wrote as
    /// a reference must not end up matched as `&&str`.
    #[test]
    fn test_try_from_function_info_strips_one_level_of_reference_per_param() {
        let info = SpyModuleInfo::try_from(&function_info(syn::parse_quote! {
            fn get_user(id: String, uuid: &str, count: &mut usize) -> String {
                todo!()
            }
        }))
        .expect("conversion should succeed for a standalone function");

        assert_eq!(render(&info.param_idents), vec!["id", "uuid", "count"]);
        assert_eq!(
            render(&info.param_types_unreferenced),
            vec!["String", "str", "usize"]
        );
    }

    #[test]
    fn test_try_from_function_info_keeps_mut_bindings_but_drops_the_mut() {
        let info = SpyModuleInfo::try_from(&function_info(syn::parse_quote! {
            fn get_user(mut id: String) -> String {
                todo!()
            }
        }))
        .expect("conversion should succeed for a `mut` binding");

        assert_eq!(render(&info.param_idents), vec!["id"]);
    }

    #[test]
    fn test_try_from_function_info_zero_params() {
        let info = SpyModuleInfo::try_from(&function_info(syn::parse_quote! {
            fn ping() {}
        }))
        .expect("conversion should succeed for a function with no parameters");

        assert!(info.param_idents.is_empty());
        assert!(info.param_types_unreferenced.is_empty());
    }

    #[test]
    fn test_try_from_function_info_rejects_a_destructuring_param() {
        let result = SpyModuleInfo::try_from(&function_info(syn::parse_quote! {
            fn foo((a, b): (i32, i32)) {}
        }));

        let Err(error) = result else {
            panic!(
                "a destructuring parameter should be rejected: a matcher needs one name per parameter"
            );
        };
        assert!(
            error.to_string().contains("#[spyable]"),
            "the error should name the attribute that was applied, got: {error}"
        );
    }
}
