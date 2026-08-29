use crate::scheme::common::names::{snake_case_path, snake_to_pascal_case};

/// Builds the spy module name, e.g. `UserService` + `get_user` -> `user_service__get_user_spy_module`.
pub fn build_module_name(struct_name: &syn::TypePath, method_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!("{}__{method_name}_spy_module", snake_case_path(struct_name)),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the `thread_local` store name, e.g. `UserService` + `get_user` -> `USER_SERVICE_GET_USER_SPY_STORE`.
pub fn build_store_name(struct_name: &syn::TypePath, method_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}_{}_SPY_STORE",
            snake_case_path(struct_name).to_uppercase(),
            method_name.to_string().to_uppercase()
        ),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the accessor method name, e.g. `get_user` -> `get_user_spy`.
pub fn build_accessor_name(method_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!("{method_name}_spy"),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the interface struct name, e.g. `UserService` + `get_user` -> `UserServiceGetUserSpyInterface`.
pub fn build_interface_name(
    struct_name: &syn::TypePath,
    method_name: &syn::Ident,
) -> syn::Result<syn::Ident> {
    build_pascal_case_name(struct_name, method_name, "SpyInterface")
}

/// Builds the matcher struct name, e.g. `UserService` + `get_user` -> `UserServiceGetUserMatcher`.
pub fn build_matcher_name(
    struct_name: &syn::TypePath,
    method_name: &syn::Ident,
) -> syn::Result<syn::Ident> {
    build_pascal_case_name(struct_name, method_name, "Matcher")
}

/// Builds the name of the matcher's `Params<'a>` wrapper struct, e.g. `UserService` + `get_user`
/// -> `UserServiceGetUserMatcherParams`.
pub fn build_params_name(
    struct_name: &syn::TypePath,
    method_name: &syn::Ident,
) -> syn::Result<syn::Ident> {
    build_pascal_case_name(struct_name, method_name, "MatcherParams")
}

/// Builds a PascalCase name of the form `{Struct}{Method}{suffix}` from the struct path's last
/// segment and the method name.
fn build_pascal_case_name(
    struct_name: &syn::TypePath,
    method_name: &syn::Ident,
    suffix: &str,
) -> syn::Result<syn::Ident> {
    let last_segment = struct_name.path.segments.last().ok_or_else(|| {
        syn::Error::new_spanned(
            struct_name,
            "Struct path has no segments. This is an error in fnmock. Please report this bug.",
        )
    })?;

    Ok(syn::Ident::new(
        &format!(
            "{}{}{suffix}",
            last_segment.ident,
            snake_to_pascal_case(&method_name.to_string())
        ),
        proc_macro2::Span::mixed_site(),
    ))
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
            "user_service__get_user_spy_module"
        );
    }

    #[test]
    fn test_build_module_name_mangles_every_path_segment() {
        let struct_name: syn::TypePath = syn::parse_quote!(a::Config);
        let other_struct_name: syn::TypePath = syn::parse_quote!(b::Config);
        let method_name: syn::Ident = syn::parse_quote!(basic);
        assert_eq!(
            build_module_name(&struct_name, &method_name).to_string(),
            "a__config__basic_spy_module"
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
            "USER_SERVICE_GET_USER_SPY_STORE"
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
            "get_user_spy"
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
            "UserServiceGetUserSpyInterface"
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
            "ConfigBasicSpyInterface"
        );
    }

    #[test]
    fn test_build_matcher_name() {
        let struct_name: syn::TypePath = syn::parse_quote!(UserService);
        let method_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_matcher_name(&struct_name, &method_name)
                .expect("valid struct path")
                .to_string(),
            "UserServiceGetUserMatcher"
        );
    }

    #[test]
    fn test_build_params_name() {
        let struct_name: syn::TypePath = syn::parse_quote!(UserService);
        let method_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_params_name(&struct_name, &method_name)
                .expect("valid struct path")
                .to_string(),
            "UserServiceGetUserMatcherParams"
        );
    }
}
