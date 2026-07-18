use crate::extract::{
    generics::{
        key_array::build_generic_key_array, params::extract_generic_type_and_const_params,
        types::extract_generic_itents_from_generic_params,
    },
    item_impl::info::ImplItemFnGenericInfo,
};

/// Extract the generic type parameters (e.g. `T: Display + 'static`, `U: 'static`) from a impl block method.
///
/// The generics of the struct and method are combined, in the order of struct generics followed by method generics.
pub fn extract_generic_impl_info(
    item_impl: &syn::ItemImpl,
    method: &syn::ImplItemFn,
) -> syn::Result<Option<ImplItemFnGenericInfo>> {
    let struct_generic_params = extract_generic_type_and_const_params(&item_impl.generics)?;
    let method_generic_params = extract_generic_type_and_const_params(&method.sig.generics)?;
    let type_params = struct_generic_params.combine(&method_generic_params);

    if type_params.is_empty() {
        return Ok(None);
    }

    let struct_idents = extract_generic_itents_from_generic_params(&struct_generic_params)?;
    let method_idents = extract_generic_itents_from_generic_params(&method_generic_params)?;
    let idents = extract_generic_itents_from_generic_params(&type_params)?;

    let struct_generic_keys = build_generic_key_array(&struct_generic_params)?;
    let method_generic_keys = build_generic_key_array(&method_generic_params)?;
    let generic_keys = build_generic_key_array(&type_params)?;

    Ok(Some(ImplItemFnGenericInfo {
        count: type_params.len(),

        generic_params: type_params.into_generic_params(),
        _struct_generic_params: struct_generic_params.into_generic_params(),
        method_generic_params: method_generic_params.into_generic_params(),

        idents,
        _struct_idents: struct_idents,
        _method_idents: method_idents,

        generic_keys,
        _struct_generic_keys: struct_generic_keys,
        _method_generic_keys: method_generic_keys,
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

    #[test]
    fn test_no_generics_anywhere_returns_none() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl Foo {
                fn bar(x: i32) {}
            }
        };
        let method = first_method(&item_impl);

        let result = extract_generic_impl_info(&item_impl, &method);

        let Ok(None) = result else {
            panic!("expected Ok(None) when neither struct nor method has generics");
        };
    }

    #[test]
    fn test_struct_and_method_generics_are_combined() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl<S> Foo<S> {
                fn bar<M>(x: M) -> S {
                    todo!()
                }
            }
        };
        let method = first_method(&item_impl);

        let result = extract_generic_impl_info(&item_impl, &method);

        let Ok(Some(info)) = result else {
            panic!("expected Ok(Some(_)) for struct + method generics");
        };
        assert_eq!(info.count, 2);
        assert_eq!(as_strings(&info.idents), vec!["S", "M"]);
        assert_eq!(as_strings(&info.method_generic_params), vec!["M"]);
    }

    #[test]
    fn test_only_method_generics() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl Foo {
                fn bar<M>(x: M) {}
            }
        };
        let method = first_method(&item_impl);

        let result = extract_generic_impl_info(&item_impl, &method);

        let Ok(Some(info)) = result else {
            panic!("expected Ok(Some(_)) for method-only generics");
        };
        assert_eq!(info.count, 1);
        assert_eq!(as_strings(&info.idents), vec!["M"]);
        assert_eq!(as_strings(&info.method_generic_params), vec!["M"]);
    }

    #[test]
    fn test_only_struct_generics() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl<S> Foo<S> {
                fn bar(x: S) {}
            }
        };
        let method = first_method(&item_impl);

        let result = extract_generic_impl_info(&item_impl, &method);

        let Ok(Some(info)) = result else {
            panic!("expected Ok(Some(_)) for struct-only generics");
        };
        assert_eq!(info.count, 1);
        assert_eq!(as_strings(&info.idents), vec!["S"]);
        assert!(
            info.method_generic_params.is_empty(),
            "expected method_generic_params to be empty when the method has no generics"
        );
    }

    #[test]
    fn test_const_generic_on_struct() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl<const N: usize> Foo<N> {
                fn bar() {}
            }
        };
        let method = first_method(&item_impl);

        let result = extract_generic_impl_info(&item_impl, &method);

        let Ok(Some(info)) = result else {
            panic!("expected Ok(Some(_)) for a struct const generic");
        };
        assert_eq!(info.count, 1);
        assert_eq!(as_strings(&info.idents), vec!["N"]);
    }
}
