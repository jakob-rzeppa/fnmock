use quote::quote;

use crate::scheme::common::generic_scheme::GenericScheme;

pub fn build_implementation_getter(
    store_name: &syn::Ident,
    fn_closure_trait: &syn::TraitBound,
    generic_scheme: Option<&GenericScheme>,
) -> proc_macro2::TokenStream {
    if let Some(generic_scheme) = generic_scheme {
        let generic_params = &generic_scheme.params;
        let generic_keys = &generic_scheme.keys;
        quote! {
            pub(super) fn implementation<#(#generic_params),*>() -> Option<::std::rc::Rc<Box<dyn #fn_closure_trait>>> {
                #store_name.with_borrow(|fake| {
                    fake.get_for::<Box<dyn #fn_closure_trait>>([#(#generic_keys),*])
                })
            }
        }
    } else {
        quote! {
            pub(super) fn implementation() -> Option<::std::rc::Rc<dyn #fn_closure_trait>> {
                #store_name.with(|store| store.borrow().get())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_non_generic() {
        let store_name: syn::Ident = parse_quote!(MY_FUNCTION_STORE);
        let fn_closure_trait: syn::TraitBound = parse_quote!(Fn(i32) -> bool);

        let res = build_implementation_getter(&store_name, &fn_closure_trait, None);

        let expected = quote! {
            pub(super) fn implementation() -> Option<::std::rc::Rc<dyn Fn(i32) -> bool>> {
                MY_FUNCTION_STORE.with(|store| store.borrow().get())
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_with_single_key() {
        let store_name: syn::Ident = parse_quote!(MY_FUNCTION_STORE);
        let fn_closure_trait: syn::TraitBound = parse_quote!(Fn(T) -> bool);
        let generic_scheme = GenericScheme {
            params: vec![parse_quote!(T)],
            idents: vec![parse_quote!(T)],
            idents_without_const_generics: vec![parse_quote!(T)],
            keys: vec![parse_quote!(::std::any::TypeId::of::<T>())],
        };

        let res =
            build_implementation_getter(&store_name, &fn_closure_trait, Some(&generic_scheme));

        let expected = quote! {
            pub(super) fn implementation<T>() -> Option<::std::rc::Rc<Box<dyn Fn(T) -> bool>>> {
                MY_FUNCTION_STORE.with_borrow(|fake| {
                    fake.get_for::<Box<dyn Fn(T) -> bool>>([::std::any::TypeId::of::<T>()])
                })
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_with_multiple_keys() {
        let store_name: syn::Ident = parse_quote!(MY_FUNCTION_STORE);
        let fn_closure_trait: syn::TraitBound = parse_quote!(Fn());
        let generic_scheme = GenericScheme {
            params: vec![parse_quote!(T: Display), parse_quote!(const C: u64)],
            idents: vec![parse_quote!(T), parse_quote!(C)],
            idents_without_const_generics: vec![parse_quote!(T)],
            keys: vec![parse_quote!(::std::any::TypeId::of::<T>()), parse_quote!(C)],
        };

        let res =
            build_implementation_getter(&store_name, &fn_closure_trait, Some(&generic_scheme));

        let expected = quote! {
            pub(super) fn implementation<T: Display, const C: u64>() -> Option<::std::rc::Rc<Box<dyn Fn()>>> {
                MY_FUNCTION_STORE.with_borrow(|fake| {
                    fake.get_for::<Box<dyn Fn()>>([::std::any::TypeId::of::<T>(), C])
                })
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }
}
