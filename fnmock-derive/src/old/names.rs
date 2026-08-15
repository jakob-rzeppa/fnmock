//! The naming scheme for generated items.
//!
//! Every generated name is derived here rather than at its use site, so that the pieces of a
//! single fake — module, store, interface struct, accessor — are guaranteed to agree on what they
//! call each other.

/// Which kind of test double a name is being built for.
///
/// Fakes and spies exist today; mocks are planned, and this is the seam they will slot into so
/// that each kind gets its own set of generated names without colliding.
///
/// It doubles as the identity of the attribute being expanded, so that the extraction code shared
/// between the kinds can name the right attribute in its error messages — see
/// [`NameType::attribute_name`].
#[derive(Clone, Copy)]
pub enum NameType {
    /// A fake: a user-supplied replacement implementation.
    Fake,
    /// A spy: the real implementation, with its calls recorded and matched against expectations.
    Spy,
}

impl NameType {
    /// The attribute this kind is expanded from, spelled the way a user writes it.
    ///
    /// Used by the extraction code both attributes share, so that a rejected construct is
    /// reported against the attribute that was actually applied.
    pub fn attribute_name(&self) -> &'static str {
        match self {
            NameType::Fake => "#[fakeable]",
            NameType::Spy => "#[spyable]",
        }
    }

    fn suffix_module(&self) -> &'static str {
        match self {
            NameType::Fake => "fake_module",
            NameType::Spy => "spy_module",
        }
    }

    fn suffix_access_function(&self) -> &'static str {
        match self {
            NameType::Fake => "fake",
            NameType::Spy => "spy",
        }
    }

    fn suffix_store(&self) -> &'static str {
        match self {
            NameType::Fake => "FAKE_STORE",
            NameType::Spy => "SPY_STORE",
        }
    }

    fn suffix_interface_struct(&self) -> &'static str {
        match self {
            NameType::Fake => "FakeInterface",
            NameType::Spy => "SpyInterface",
        }
    }

    fn suffix_matcher(&self) -> &'static str {
        match self {
            NameType::Fake | NameType::Spy => "Matcher",
        }
    }
}

/// Builds the module name for a function fake, spy etc.
///
/// For a function named `get_user`, this will generate `get_user_fake_module`.
pub fn build_module_name(fn_name: &syn::Ident, name_type: NameType) -> syn::Ident {
    syn::Ident::new(
        &format!("{}_{}", fn_name, name_type.suffix_module()),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the access function name for a function fake, spy etc.
///
/// This is used for methods and standalone functions.
///
/// For a function named `get_user`, this will generate `get_user_fake`.
pub fn build_access_function_name(fn_name: &syn::Ident, name_type: NameType) -> syn::Ident {
    syn::Ident::new(
        &format!("{}_{}", fn_name, name_type.suffix_access_function()),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the module name for an impl block fake, spy etc.
///
/// For a struct named `UserService` and a method named `get_user`, this will generate `user_service__get_user_fake_module`.
///
/// The doubled `__` delimiters keep this name from colliding with other module names, since the `__`
/// is generally not used in the middle of a function name.
pub fn build_impl_module_name(
    struct_name: &syn::Ident,
    method_name: &syn::Ident,
    name_type: NameType,
) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}__{}_{}",
            pascal_to_snake_case(&struct_name.to_string()),
            method_name,
            name_type.suffix_module()
        ),
        proc_macro2::Span::mixed_site(),
    )
}

/// Build the store name for a function fake, spy etc.
///
/// For a function named `get_user`, this will generate `GET_USER_FAKE_STORE`.
pub fn build_store_name(fn_name: &syn::Ident, name_type: NameType) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}_{}",
            fn_name.to_string().to_uppercase(),
            name_type.suffix_store()
        ),
        proc_macro2::Span::mixed_site(),
    )
}

/// Build the store name for an impl block fake, spy etc.
///
/// For a struct named `UserService` and a method named `get_user`, this will generate `USER_SERVICE_GET_USER_FAKE_STORE`.
pub fn build_impl_store_name(
    struct_name: &syn::Ident,
    method_name: &syn::Ident,
    name_type: NameType,
) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}_{}_{}",
            pascal_to_snake_case(&struct_name.to_string()).to_uppercase(),
            method_name.to_string().to_uppercase(),
            name_type.suffix_store()
        ),
        proc_macro2::Span::mixed_site(),
    )
}

/// Build the interface struct name for a function fake, spy etc.
///
/// For a function named `get_user`, this will generate `GetUserFakeInterface`.
pub fn build_interface_struct_name(fn_name: &syn::Ident, name_type: NameType) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}{}",
            snake_to_pascal_case(&fn_name.to_string()),
            name_type.suffix_interface_struct()
        ),
        proc_macro2::Span::mixed_site(),
    )
}

/// Build the interface struct name for an impl block fake, spy etc.
///
/// For a struct named `UserService` and a method named `get_user`, this will generate `UserServiceGetUserFakeInterface`.
pub fn build_impl_interface_struct_name(
    struct_name: &syn::Ident,
    method_name: &syn::Ident,
    name_type: NameType,
) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}{}{}",
            struct_name,
            snake_to_pascal_case(&method_name.to_string()),
            name_type.suffix_interface_struct()
        ),
        proc_macro2::Span::mixed_site(),
    )
}

/// Build the matcher name for a function spy, mock etc.
///
/// The matcher lives inside the generated module, so this only has to stay clear of the other
/// items generated beside it.
///
/// For a function named `get_user`, this will generate `GetUserMatcher`.
pub fn build_matcher_name(fn_name: &syn::Ident, name_type: NameType) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}{}",
            snake_to_pascal_case(&fn_name.to_string()),
            name_type.suffix_matcher()
        ),
        proc_macro2::Span::mixed_site(),
    )
}

/// Helper: Convert snake_case to PascalCase
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

/// Helper: Convert PascalCase to snake_case
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
        let fn_name = syn::Ident::new("get_user", proc_macro2::Span::call_site());
        let module_name = build_module_name(&fn_name, NameType::Fake);
        assert_eq!(module_name.to_string(), "get_user_fake_module");
    }

    #[test]
    fn test_build_impl_module_name() {
        let struct_name = syn::Ident::new("UserService", proc_macro2::Span::call_site());
        let method_name = syn::Ident::new("get_user", proc_macro2::Span::call_site());
        let module_name = build_impl_module_name(&struct_name, &method_name, NameType::Fake);
        assert_eq!(
            module_name.to_string(),
            "user_service__get_user_fake_module"
        );
    }

    #[test]
    fn test_build_store_name() {
        let fn_name = syn::Ident::new("get_user", proc_macro2::Span::call_site());
        let store_name = build_store_name(&fn_name, NameType::Fake);
        assert_eq!(store_name.to_string(), "GET_USER_FAKE_STORE");
    }

    #[test]
    fn test_build_impl_store_name() {
        let struct_name = syn::Ident::new("UserService", proc_macro2::Span::call_site());
        let method_name = syn::Ident::new("get_user", proc_macro2::Span::call_site());
        let store_name = build_impl_store_name(&struct_name, &method_name, NameType::Fake);
        assert_eq!(store_name.to_string(), "USER_SERVICE_GET_USER_FAKE_STORE");
    }

    #[test]
    fn test_build_interface_struct_name() {
        let fn_name = syn::Ident::new("get_user", proc_macro2::Span::call_site());
        let interface_struct_name = build_interface_struct_name(&fn_name, NameType::Fake);
        assert_eq!(interface_struct_name.to_string(), "GetUserFakeInterface");
    }

    #[test]
    fn test_build_impl_interface_struct_name() {
        let struct_name = syn::Ident::new("UserService", proc_macro2::Span::call_site());
        let method_name = syn::Ident::new("get_user", proc_macro2::Span::call_site());
        let interface_struct_name =
            build_impl_interface_struct_name(&struct_name, &method_name, NameType::Fake);
        assert_eq!(
            interface_struct_name.to_string(),
            "UserServiceGetUserFakeInterface"
        );
    }

    #[test]
    fn test_build_matcher_name() {
        let fn_name = syn::Ident::new("get_user", proc_macro2::Span::call_site());
        let matcher_name = build_matcher_name(&fn_name, NameType::Spy);
        assert_eq!(matcher_name.to_string(), "GetUserMatcher");
    }

    /// A spy's names have to agree with what the spy generators emit, and must not collide with
    /// the fake's names for the same function.
    #[test]
    fn test_spy_names_for_a_free_function() {
        let fn_name = syn::Ident::new("get_user", proc_macro2::Span::call_site());

        assert_eq!(
            build_module_name(&fn_name, NameType::Spy).to_string(),
            "get_user_spy_module"
        );
        assert_eq!(
            build_access_function_name(&fn_name, NameType::Spy).to_string(),
            "get_user_spy"
        );
        assert_eq!(
            build_store_name(&fn_name, NameType::Spy).to_string(),
            "GET_USER_SPY_STORE"
        );
        assert_eq!(
            build_interface_struct_name(&fn_name, NameType::Spy).to_string(),
            "GetUserSpyInterface"
        );
    }

    #[test]
    fn test_attribute_name_matches_the_attribute_the_user_writes() {
        assert_eq!(NameType::Fake.attribute_name(), "#[fakeable]");
        assert_eq!(NameType::Spy.attribute_name(), "#[spyable]");
    }
}
