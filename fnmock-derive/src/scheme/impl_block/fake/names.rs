/// Builds the fake module name, e.g. `UserService` + `get_user` -> `user_service__get_user_fake_module`.
pub fn build_module_name(struct_name: &syn::Ident, method_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}__{method_name}_fake_module",
            pascal_to_snake_case(&struct_name.to_string())
        ),
        proc_macro2::Span::mixed_site(),
    )
}

/// Builds the `thread_local` store name, e.g. `UserService` + `get_user` -> `USER_SERVICE_GET_USER_FAKE_STORE`.
pub fn build_store_name(struct_name: &syn::Ident, method_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{}_{}_FAKE_STORE",
            pascal_to_snake_case(&struct_name.to_string()).to_uppercase(),
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
pub fn build_interface_name(struct_name: &syn::Ident, method_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "{struct_name}{}FakeInterface",
            snake_to_pascal_case(&method_name.to_string())
        ),
        proc_macro2::Span::mixed_site(),
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
        let struct_name: syn::Ident = syn::parse_quote!(UserService);
        let method_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_module_name(&struct_name, &method_name).to_string(),
            "user_service__get_user_fake_module"
        );
    }

    #[test]
    fn test_build_store_name() {
        let struct_name: syn::Ident = syn::parse_quote!(UserService);
        let method_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_store_name(&struct_name, &method_name).to_string(),
            "USER_SERVICE_GET_USER_FAKE_STORE"
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
        let struct_name: syn::Ident = syn::parse_quote!(UserService);
        let method_name: syn::Ident = syn::parse_quote!(get_user);
        assert_eq!(
            build_interface_name(&struct_name, &method_name).to_string(),
            "UserServiceGetUserFakeInterface"
        );
    }
}
