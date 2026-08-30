//! Case and path-mangling helpers shared by the fake and spy name builders.

/// Mangles every segment of a type path into one snake_case string, e.g. `a::Config` ->
/// `a__config`. The double underscore is used to separate segments to avoid collisions, e.g. `a::Config` and `a_config` would otherwise both mangle to `a_config`. Each segment's generic
/// arguments (if any) are folded in too, e.g. `Foo<u8>` -> `foo_u8`, so that two impl blocks for
/// the same struct at different concrete type arguments (e.g. `Foo<u8>` and `Foo<u16>`) don't
/// mangle to the same identifier.
pub fn snake_case_path(struct_name: &syn::TypePath) -> String {
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

pub fn pascal_to_snake_case(s: &str) -> String {
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

/// Builds a PascalCase name of the form `{Struct}{Method}{suffix}` from the struct path's last
/// segment and the method name, e.g. `UserService` + `get_user` + `Matcher` ->
/// `UserServiceGetUserMatcher`.
///
/// Only the last segment is used: these names are all emitted inside the method's own module, so
/// the leading path segments that [`snake_case_path`] needs for collision avoidance would only add
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
            last_segment.ident,
            snake_to_pascal_case(&method_name.to_string())
        ),
        proc_macro2::Span::mixed_site(),
    ))
}
