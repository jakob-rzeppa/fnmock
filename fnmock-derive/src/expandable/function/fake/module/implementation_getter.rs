use quote::quote;

pub fn build_implementation_getter(
    store_name: &syn::Ident,
    fn_closure_trait: &syn::TraitBound,
    generic_params: Option<&[syn::GenericParam]>,
    generic_keys: Option<&[syn::Expr]>,
) -> proc_macro2::TokenStream {
    if let (Some(generic_keys), Some(generic_params)) = (generic_keys, generic_params) {
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

        let res = build_implementation_getter(&store_name, &fn_closure_trait, None, None);

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
        let generic_params: Vec<syn::GenericParam> = vec![parse_quote!(T)];
        let generic_keys: Vec<syn::Expr> = vec![parse_quote!(::std::any::TypeId::of::<T>())];

        let res = build_implementation_getter(
            &store_name,
            &fn_closure_trait,
            Some(&generic_params),
            Some(&generic_keys),
        );

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
        let generic_params: Vec<syn::GenericParam> =
            vec![parse_quote!(T: Display), parse_quote!(const C: u64)];
        let generic_keys: Vec<syn::Expr> =
            vec![parse_quote!(::std::any::TypeId::of::<T>()), parse_quote!(C)];

        let res = build_implementation_getter(
            &store_name,
            &fn_closure_trait,
            Some(&generic_params),
            Some(&generic_keys),
        );

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
