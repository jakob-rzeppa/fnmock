use crate::scheme::common::names::{build_pascal_case_name, mangle_type_path};
use syn::ext::IdentExt;

/// Builds the fake module name, e.g. `UserService` + `get_user` -> `UserService__get_user_fake_method_module`.
///
/// Located at `method_name` so that resolution errors on the module path (e.g. when the impl block
/// is declared inside a function body) point at the method rather than the attribute.
pub fn build_module_name(struct_name: &syn::TypePath, method_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}__{}_fake_method_module",
            mangle_type_path(struct_name),
            method_name.unraw()
        ),
        proc_macro2::Span::mixed_site().located_at(method_name.span()),
    )
}

/// Builds the `thread_local` store name, e.g. `UserService` + `get_user` -> `USERSERVICE_GET_USER_FAKE_STORE`.
pub fn build_store_name(struct_name: &syn::TypePath, method_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}_{}_FAKE_STORE",
            mangle_type_path(struct_name).to_uppercase(),
            method_name.unraw().to_string().to_uppercase()
        ),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the accessor method name, e.g. `get_user` -> `get_user_fake`.
pub fn build_accessor_name(method_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!("{}_fake", method_name.unraw()),
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
            "UserService__get_user_fake_method_module"
        );
    }

    #[test]
    fn test_build_module_name_mangles_every_path_segment() {
        let struct_name: syn::TypePath = syn::parse_quote!(a::Config);
        let other_struct_name: syn::TypePath = syn::parse_quote!(b::Config);
        let method_name: syn::Ident = syn::parse_quote!(basic);
        assert_eq!(
            build_module_name(&struct_name, &method_name).to_string(),
            "a__Config__basic_fake_method_module"
        );
        assert_ne!(
            build_module_name(&struct_name, &method_name),
            build_module_name(&other_struct_name, &method_name)
        );
    }

    #[test]
    fn test_build_module_name_has_method_marker() {
        // A free fn `foo__bar` gets `foo__bar_fake_module`, so the `_method` marker keeps the two apart.
        let struct_name: syn::TypePath = syn::parse_quote!(foo);
        let method_name: syn::Ident = syn::parse_quote!(bar);
        assert_eq!(
            build_module_name(&struct_name, &method_name).to_string(),
            "foo__bar_fake_method_module"
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
            "USERSERVICE_GET_USER_FAKE_STORE"
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
