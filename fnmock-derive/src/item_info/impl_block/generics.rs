//! Extraction of an impl block's struct-level and method-level generic parameters, kept apart so
//! [`ItemImplInfo`](super::info::ItemImplInfo) can store the struct's generics once instead of
//! once per method.

use crate::item_info::{
    generics::{
        key_array::build_generic_key_array, params::extract_generic_type_and_const_params,
        types::extract_generic_idents_from_generic_params,
    },
    impl_block::info::{ImplMethodGenericInfo, StructGenericInfo},
};

/// Extract the struct's own generic type/const parameters (e.g. `impl<S> Foo<S>`).
///
/// Returns `Ok(None)` when the struct has nothing to key a fake by.
///
/// # Errors
///
/// Returns a spanned error if a generic parameter carries a non-`'static` lifetime bound.
pub fn extract_struct_generic_info(
    item_impl: &syn::ItemImpl,
) -> syn::Result<Option<StructGenericInfo>> {
    let struct_generic_params = extract_generic_type_and_const_params(&item_impl.generics)?;

    if struct_generic_params.is_empty() {
        return Ok(None);
    }

    let idents = extract_generic_idents_from_generic_params(&struct_generic_params)?;
    let generic_keys = build_generic_key_array(&struct_generic_params)?;

    Ok(Some(StructGenericInfo {
        generic_params: struct_generic_params.into_generic_params(),
        idents,
        generic_keys,
    }))
}

/// Extract a single method's own generic type/const parameters (e.g. `fn bar<M>(..)`).
///
/// Returns `Ok(None)` when the method has nothing to key a fake by.
///
/// # Errors
///
/// Returns a spanned error if a generic parameter carries a non-`'static` lifetime bound.
pub fn extract_method_generic_info(
    method: &syn::ImplItemFn,
) -> syn::Result<Option<ImplMethodGenericInfo>> {
    let method_generic_params = extract_generic_type_and_const_params(&method.sig.generics)?;

    if method_generic_params.is_empty() {
        return Ok(None);
    }

    let idents = extract_generic_idents_from_generic_params(&method_generic_params)?;
    let generic_keys = build_generic_key_array(&method_generic_params)?;

    Ok(Some(ImplMethodGenericInfo {
        generic_params: method_generic_params.into_generic_params(),
        idents,
        generic_keys,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    fn as_strings<T: ToTokens>(items: &[T]) -> Vec<String> {
        items
            .iter()
            .map(|i| i.to_token_stream().to_string())
            .collect()
    }

    fn first_method(item_impl: &syn::ItemImpl) -> syn::ImplItemFn {
        item_impl
            .items
            .iter()
            .find_map(|item| match item {
                syn::ImplItem::Fn(f) => Some(f.clone()),
                _ => None,
            })
            .expect("impl should contain a method")
    }

    mod extract_struct_generic_info_tests {
        use super::*;

        #[test]
        fn test_no_struct_generics_returns_none() {
            let item_impl: syn::ItemImpl = syn::parse_quote! {
                impl Foo {
                    fn bar(x: i32) {}
                }
            };

            let result = extract_struct_generic_info(&item_impl);

            let Ok(None) = result else {
                panic!("expected Ok(None) when the struct has no generics");
            };
        }

        #[test]
        fn test_struct_generics_are_extracted_independent_of_method_generics() {
            let item_impl: syn::ItemImpl = syn::parse_quote! {
                impl<S> Foo<S> {
                    fn bar<M>(x: M) -> S {
                        todo!()
                    }
                }
            };

            let result = extract_struct_generic_info(&item_impl);

            let Ok(Some(info)) = result else {
                panic!("expected Ok(Some(_)) for a generic struct");
            };
            assert_eq!(as_strings(&info.idents), vec!["S"]);
        }

        #[test]
        fn test_only_method_generics_returns_none_for_struct() {
            let item_impl: syn::ItemImpl = syn::parse_quote! {
                impl Foo {
                    fn bar<M>(x: M) {}
                }
            };

            let result = extract_struct_generic_info(&item_impl);

            let Ok(None) = result else {
                panic!("expected Ok(None) for a non-generic struct with a generic method");
            };
        }

        #[test]
        fn test_const_generic_on_struct() {
            let item_impl: syn::ItemImpl = syn::parse_quote! {
                impl<const N: usize> Foo<N> {
                    fn bar() {}
                }
            };

            let result = extract_struct_generic_info(&item_impl);

            let Ok(Some(info)) = result else {
                panic!("expected Ok(Some(_)) for a struct const generic");
            };
            assert_eq!(as_strings(&info.idents), vec!["N"]);
        }
    }

    mod extract_method_generic_info_tests {
        use super::*;

        #[test]
        fn test_no_method_generics_returns_none() {
            let item_impl: syn::ItemImpl = syn::parse_quote! {
                impl<S> Foo<S> {
                    fn bar(x: S) {}
                }
            };
            let method = first_method(&item_impl);

            let result = extract_method_generic_info(&method);

            let Ok(None) = result else {
                panic!(
                    "expected Ok(None) when the method has no generics, regardless of the struct's"
                );
            };
        }

        #[test]
        fn test_method_generics_are_extracted_independent_of_struct_generics() {
            let item_impl: syn::ItemImpl = syn::parse_quote! {
                impl<S> Foo<S> {
                    fn bar<M>(x: M) -> S {
                        todo!()
                    }
                }
            };
            let method = first_method(&item_impl);

            let result = extract_method_generic_info(&method);

            let Ok(Some(info)) = result else {
                panic!("expected Ok(Some(_)) for a generic method");
            };
            assert_eq!(as_strings(&info.idents), vec!["M"]);
        }
    }
}
