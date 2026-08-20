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
    let last_segment = struct_name.path.segments.last().ok_or_else(|| {
        syn::Error::new_spanned(
            struct_name,
            "Struct path has no segments. This is an error in fnmock. Please report this bug.",
        )
    })?;

    Ok(syn::Ident::new(
        &format!(
            "{}{}FakeInterface",
            last_segment.ident,
            snake_to_pascal_case(&method_name.to_string())
        ),
        proc_macro2::Span::mixed_site(),
    ))
}

/// Mangles every segment of a type path into one snake_case string, e.g. `a::Config` ->
/// `a__config`. The double underscore is used to separate segments to avoid collisions, e.g. `a::Config` and `a_config` would otherwise both mangle to `a_config`. Each segment's generic
/// arguments (if any) are folded in too, e.g. `Foo<u8>` -> `foo_u8`, so that two impl blocks for
/// the same struct at different concrete type arguments (e.g. `Foo<u8>` and `Foo<u16>`) don't
/// mangle to the same identifier.
fn snake_case_path(struct_name: &syn::TypePath) -> String {
    struct_name
        .path
        .segments
        .iter()
        .map(|segment| {
            let base = pascal_to_snake_case(&segment.ident.to_string());
            match mangle_generic_arguments(&segment.arguments) {
                Some(suffix) => format!("{base}_{suffix}"),
                None => base,
            }
        })
        .collect::<Vec<_>>()
        .join("__")
}

/// Mangles a path segment's generic arguments (if any) into a snake_case suffix, e.g. `<u8>` ->
/// `u8`, `<u8, U>` -> `u8_u`. Returns `None` for `PathArguments::None` so non-generic segments'
/// mangled names are unaffected.
fn mangle_generic_arguments(arguments: &syn::PathArguments) -> Option<String> {
    if matches!(arguments, syn::PathArguments::None) {
        return None;
    }

    let tokens = quote::ToTokens::to_token_stream(arguments).to_string();
    let mangled: String = tokens
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();

    Some(
        mangled
            .split('_')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("_"),
    )
}

fn snake_to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn pascal_to_snake_case(s: &str) -> String {
    let mut snake = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                snake.push('_');
            }
            snake.push(c.to_ascii_lowercase());
        } else {
            snake.push(c);
        }
    }
    snake
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
