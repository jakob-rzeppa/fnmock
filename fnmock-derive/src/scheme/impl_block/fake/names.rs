use crate::scheme::common::names::{build_pascal_case_name, snake_case_path};

/// Builds the fake module name, e.g. `UserService` + `get_user` -> `user_service__get_user_fake_module`.
pub fn build_module_name(struct_name: &syn::TypePath, method_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}__{method_name}_fake_module",
            snake_case_path(struct_name)
        ),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the `thread_local` store name, e.g. `UserService` + `get_user` -> `USER_SERVICE_GET_USER_FAKE_STORE`.
pub fn build_store_name(struct_name: &syn::TypePath, method_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}_{}_FAKE_STORE",
            snake_case_path(struct_name).to_uppercase(),
            method_name.to_string().to_uppercase()
        ),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the accessor method name, e.g. `get_user` -> `get_user_fake`.
pub fn build_accessor_name(method_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!("{method_name}_fake"),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the interface struct name, e.g. `UserService` + `get_user` -> `UserServiceGetUserFakeInterface`.
pub fn build_interface_name(
    struct_name: &syn::TypePath,
    method_name: &syn::Ident,
) -> syn::Result<syn::Ident> {
    build_pascal_case_name(struct_name, method_name, "FakeInterface")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_module_name() {
        let struct_name: syn::TypePath = syn::parse_quote!(UserService);
        let method_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_module_name(&struct_name, &method_name).to_string(),
            "user_service__get_user_fake_module"
        );
    }

    #[test]
    fn test_build_module_name_mangles_every_path_segment() {
        let struct_name: syn::TypePath = syn::parse_quote!(a::Config);
        let other_struct_name: syn::TypePath = syn::parse_quote!(b::Config);
        let method_name: syn::Ident = syn::parse_quote!(basic);
        assert_eq!(
            build_module_name(&struct_name, &method_name).to_string(),
            "a__config__basic_fake_module"
        );
        assert_ne!(
            build_module_name(&struct_name, &method_name),
            build_module_name(&other_struct_name, &method_name)
        );
    }

    #[test]
    fn test_build_module_name_mangles_generic_arguments() {
        let struct_name: syn::TypePath = syn::parse_quote!(Foo<u8>);
        let other_struct_name: syn::TypePath = syn::parse_quote!(Foo<u16>);
        let method_name: syn::Ident = syn::parse_quote!(bar);
        assert_ne!(
            build_module_name(&struct_name, &method_name),
            build_module_name(&other_struct_name, &method_name)
        );
    }

    #[test]
    fn test_build_store_name() {
        let struct_name: syn::TypePath = syn::parse_quote!(UserService);
        let method_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_store_name(&struct_name, &method_name).to_string(),
            "USER_SERVICE_GET_USER_FAKE_STORE"
        );
    }

    #[test]
    fn test_build_store_name_mangles_generic_arguments() {
        let struct_name: syn::TypePath = syn::parse_quote!(Foo<u8>);
        let other_struct_name: syn::TypePath = syn::parse_quote!(Foo<u16>);
        let method_name: syn::Ident = syn::parse_quote!(bar);
        assert_ne!(
            build_store_name(&struct_name, &method_name),
            build_store_name(&other_struct_name, &method_name)
        );
    }

    #[test]
    fn test_build_accessor_name() {
        let method_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_accessor_name(&method_name).to_string(),
            "get_user_fake"
        );
    }

    #[test]
    fn test_build_interface_name() {
        let struct_name: syn::TypePath = syn::parse_quote!(UserService);
        let method_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_interface_name(&struct_name, &method_name)
                .expect("valid struct path")
                .to_string(),
            "UserServiceGetUserFakeInterface"
        );
    }

    #[test]
    fn test_build_interface_name_only_uses_last_path_segment() {
        let struct_name: syn::TypePath = syn::parse_quote!(a::Config);
        let method_name: syn::Ident = syn::parse_quote!(basic);
        assert_eq!(
            build_interface_name(&struct_name, &method_name)
                .expect("valid struct path")
                .to_string(),
            "ConfigBasicFakeInterface"
        );
    }
}
