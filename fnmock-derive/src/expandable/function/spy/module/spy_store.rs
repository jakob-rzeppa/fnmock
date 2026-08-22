use quote::quote;

pub fn build_spy_store(
    store_name: &syn::Ident,
    display_name: &str,
    matcher_type: &syn::Type,
    generic_count: Option<usize>,
) -> proc_macro2::TokenStream {
    if let Some(generic_count) = generic_count {
        quote! {
            thread_local! {
                static #store_name: ::std::cell::RefCell<::fnmock::generic_spy_store::GenericSpyStore<#generic_count>> =
                    ::std::cell::RefCell::new(
                        ::fnmock::generic_spy_store::GenericSpyStore::new(#display_name)
                    );
            }
        }
    } else {
        quote! {
            thread_local! {
                static #store_name: ::std::cell::RefCell<::fnmock::spy_store::SpyStore<#matcher_type>> =
                    ::std::cell::RefCell::new(::fnmock::spy_store::SpyStore::new(#display_name));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_non_generic_store() {
        let store_name: syn::Ident = parse_quote!(MY_FUNCTION_SPY_STORE);
        let display_name = "my_function";
        let matcher_type: syn::Type = parse_quote!(MyFunctionMatcher);

        let res = build_spy_store(&store_name, display_name, &matcher_type, None);

        let expected = quote! {
            thread_local! {
                static MY_FUNCTION_SPY_STORE: ::std::cell::RefCell<::fnmock::spy_store::SpyStore<MyFunctionMatcher>> =
                    ::std::cell::RefCell::new(::fnmock::spy_store::SpyStore::new("my_function"));
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_store() {
        let store_name: syn::Ident = parse_quote!(MY_FUNCTION_SPY_STORE);
        let display_name = "my_function";
        let matcher_type: syn::Type = parse_quote!(MyFunctionMatcher<T>);

        let res = build_spy_store(&store_name, display_name, &matcher_type, Some(2));

        let expected = quote! {
            thread_local! {
                static MY_FUNCTION_SPY_STORE: ::std::cell::RefCell<::fnmock::generic_spy_store::GenericSpyStore<2usize>> =
                    ::std::cell::RefCell::new(
                        ::fnmock::generic_spy_store::GenericSpyStore::new("my_function")
                    );
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }
}
