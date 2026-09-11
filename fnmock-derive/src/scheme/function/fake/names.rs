use crate::scheme::common::names::snake_to_pascal_case;

/// Builds the fake module name, e.g. `get_user` -> `get_user_fake_module`.
pub fn build_module_name(fn_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!("{fn_name}_fake_module"),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the `thread_local` store name, e.g. `get_user` -> `GET_USER_FAKE_STORE`.
pub fn build_store_name(fn_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!("{}_FAKE_STORE", fn_name.to_string().to_uppercase()),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the accessor function name, e.g. `get_user` -> `get_user_fake`.
pub fn build_accessor_name(fn_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("{fn_name}_fake"), proc_macro2::Span::mixed_site())
}

/// Builds the interface struct name, e.g. `get_user` -> `GetUserFakeInterface`.
pub fn build_interface_name(fn_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}FakeInterface",
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
            "get_user_fake_module"
        );
    }

    #[test]
    fn test_build_store_name() {
        let fn_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_store_name(&fn_name).to_string(),
            "GET_USER_FAKE_STORE"
        );
    }

    #[test]
    fn test_build_accessor_name() {
        let fn_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(build_accessor_name(&fn_name).to_string(), "get_user_fake");
    }

    #[test]
    fn test_build_interface_name() {
        let fn_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_interface_name(&fn_name).to_string(),
            "GetUserFakeInterface"
        );
    }
}
