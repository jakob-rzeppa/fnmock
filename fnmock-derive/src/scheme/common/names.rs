//! Case and path-mangling helpers shared by the fake and spy name builders.

use syn::ext::IdentExt;

/// Mangles a type path into one identifier fragment, e.g. `a::Config` -> `a__Config`, for the
/// names of the modules generated for impl methods.
///
/// Segments keep their case: converting them to snake_case would map different type names to the
/// same string (`FooBar` and `Foo_bar` both become `foo_bar`). The double underscore separates
/// segments, so `a::Config` and a type `a_Config` stay distinct. Each segment's generic arguments
/// (if any) are folded in too, e.g. `Foo<u8>` -> `Foo_u8`, so that two impl blocks for the same
/// struct at different concrete type arguments (e.g. `Foo<u8>` and `Foo<u16>`) don't mangle to the
/// same identifier.
pub fn mangle_type_path(struct_name: &syn::TypePath) -> String {
    struct_name
        .path
        .segments
        .iter()
        .map(|segment| {
            let base = segment.ident.unraw().to_string();
            match mangle_generic_arguments(&segment.arguments) {
                Some(suffix) => format!("{base}_{suffix}"),
                None => base,
            }
        })
        .collect::<Vec<_>>()
        .join("__")
}

/// Mangles a path segment's generic arguments (if any) into a suffix, e.g. `<u8>` ->
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

pub fn snake_to_pascal_case(s: &str) -> String {
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

/// Builds a PascalCase name of the form `{Struct}{Method}{suffix}` from the struct path's last
/// segment and the method name, e.g. `UserService` + `get_user` + `Matcher` ->
/// `UserServiceGetUserMatcher`.
///
/// Only the last segment is used: these names are all emitted inside the method's own module, so
/// the leading path segments that [`mangle_type_path`] needs for collision avoidance would only add
/// noise here.
///
/// # Errors
///
/// Returns a spanned error if the struct path has no segments.
pub fn build_pascal_case_name(
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
            last_segment.ident.unraw(),
            snake_to_pascal_case(&method_name.unraw().to_string())
        ),
        proc_macro2::Span::mixed_site(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mangle_type_path_keeps_case() {
        let struct_name: syn::TypePath = syn::parse_quote!(a::UserService);
        assert_eq!(mangle_type_path(&struct_name), "a__UserService");
    }

    #[test]
    fn test_mangle_type_path_distinguishes_names_that_snake_case_alike() {
        let camel: syn::TypePath = syn::parse_quote!(FooBar);
        let underscored: syn::TypePath = syn::parse_quote!(Foo_bar);
        assert_ne!(mangle_type_path(&camel), mangle_type_path(&underscored));
    }

    #[test]
    fn test_mangle_type_path_folds_in_generic_arguments() {
        let struct_name: syn::TypePath = syn::parse_quote!(Foo<u8, String>);
        assert_eq!(mangle_type_path(&struct_name), "Foo_u8_String");
    }

    #[test]
    fn test_build_pascal_case_name_strips_raw_prefixes() {
        let struct_name: syn::TypePath = syn::parse_quote!(r#Keywords);
        let method_name: syn::Ident = syn::parse_quote!(r#loop);
        assert_eq!(
            build_pascal_case_name(&struct_name, &method_name, "Matcher")
                .expect("valid struct path")
                .to_string(),
            "KeywordsLoopMatcher"
        );
    }
}
