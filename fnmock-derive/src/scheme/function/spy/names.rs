use crate::scheme::common::names::snake_to_pascal_case;

/// Builds the spy module name, e.g. `get_user` -> `get_user_spy_module`.
pub fn build_module_name(fn_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!("{fn_name}_spy_module"),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the `thread_local` store name, e.g. `get_user` -> `GET_USER_SPY_STORE`.
pub fn build_store_name(fn_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!("{}_SPY_STORE", fn_name.to_string().to_uppercase()),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the accessor function name, e.g. `get_user` -> `get_user_spy`.
pub fn build_accessor_name(fn_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("{fn_name}_spy"), proc_macro2::Span::mixed_site())
}

/// Builds the interface struct name, e.g. `get_user` -> `GetUserSpyInterface`.
pub fn build_interface_name(fn_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!("{}SpyInterface", snake_to_pascal_case(&fn_name.to_string())),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the matcher enum name, e.g. `get_user` -> `GetUserMatcher`.
pub fn build_matcher_name(fn_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!("{}Matcher", snake_to_pascal_case(&fn_name.to_string())),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the name of the matcher's `Params<'a>` wrapper struct, e.g. `get_user` ->
/// `GetUserMatcherParams`.
pub fn build_params_name(fn_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}MatcherParams",
            snake_to_pascal_case(&fn_name.to_string())
        ),
        proc_macro2::Span::mixed_site(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_module_name() {
        let fn_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_module_name(&fn_name).to_string(),
            "get_user_spy_module"
        );
    }

    #[test]
    fn test_build_store_name() {
        let fn_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(build_store_name(&fn_name).to_string(), "GET_USER_SPY_STORE");
    }

    #[test]
    fn test_build_accessor_name() {
        let fn_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(build_accessor_name(&fn_name).to_string(), "get_user_spy");
    }

    #[test]
    fn test_build_interface_name() {
        let fn_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_interface_name(&fn_name).to_string(),
            "GetUserSpyInterface"
        );
    }

    #[test]
    fn test_build_matcher_name() {
        let fn_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(build_matcher_name(&fn_name).to_string(), "GetUserMatcher");
    }

    #[test]
    fn test_build_params_name() {
        let fn_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_params_name(&fn_name).to_string(),
            "GetUserMatcherParams"
        );
    }
}
