use crate::scheme::common::names::snake_to_pascal_case;

/// Builds the mock module name, e.g. `get_user` -> `get_user_mock_module`.
pub fn build_module_name(fn_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!("{fn_name}_mock_module"),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the accessor function name, e.g. `get_user` -> `get_user_mock`.
pub fn build_accessor_name(fn_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("{fn_name}_mock"), proc_macro2::Span::mixed_site())
}

/// Builds the interface struct name, e.g. `get_user` -> `GetUserMockInterface`.
pub fn build_interface_name(fn_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}MockInterface",
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
            "get_user_mock_module"
        );
    }

    #[test]
    fn test_build_accessor_name() {
        let fn_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(build_accessor_name(&fn_name).to_string(), "get_user_mock");
    }

    #[test]
    fn test_build_interface_name() {
        let fn_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_interface_name(&fn_name).to_string(),
            "GetUserMockInterface"
        );
    }
}
