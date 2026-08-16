use quote::quote;

pub fn build_fake_store(
    store_name: &syn::Ident,
    display_name: &str,
    fn_closure_trait: &syn::TraitBound,
    generic_count: Option<usize>,
) -> proc_macro2::TokenStream {
    if let Some(generic_count) = generic_count {
        quote! {
            thread_local! {
                static #store_name: ::std::cell::RefCell<::fnmock::generic_fake_store::GenericFakeStore<#generic_count>> =
                    ::std::cell::RefCell::new(
                        ::fnmock::generic_fake_store::GenericFakeStore::new(#display_name)
                    );
            }
        }
    } else {
        quote! {
            thread_local! {
                static #store_name: ::std::cell::RefCell<::fnmock::fake_store::FakeStore<::std::rc::Rc<dyn #fn_closure_trait>>> =
                    ::std::cell::RefCell::new(::fnmock::fake_store::FakeStore::new(#display_name));
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
        let store_name: syn::Ident = parse_quote!(MY_FUNCTION_STORE);
        let display_name = "my_function";
        let fn_closure_trait: syn::TraitBound = parse_quote!(Fn(i32) -> bool);

        let res = build_fake_store(&store_name, display_name, &fn_closure_trait, None);

        let expected = quote! {
            thread_local! {
                static MY_FUNCTION_STORE: ::std::cell::RefCell<::fnmock::fake_store::FakeStore<::std::rc::Rc<dyn Fn(i32) -> bool>>> =
                    ::std::cell::RefCell::new(::fnmock::fake_store::FakeStore::new("my_function"));
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_store() {
        let store_name: syn::Ident = parse_quote!(MY_FUNCTION_STORE);
        let display_name = "my_function";
        let fn_closure_trait: syn::TraitBound = parse_quote!(Fn(i32) -> bool);

        let res = build_fake_store(&store_name, display_name, &fn_closure_trait, Some(2));

        let expected = quote! {
            thread_local! {
                static MY_FUNCTION_STORE: ::std::cell::RefCell<::fnmock::generic_fake_store::GenericFakeStore<2usize>> =
                    ::std::cell::RefCell::new(
                        ::fnmock::generic_fake_store::GenericFakeStore::new("my_function")
                    );
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }
}
