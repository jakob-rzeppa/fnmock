use quote::quote;

use crate::scheme::common::generic_scheme::GenericScheme;

pub fn build_interface_struct(
    interface_struct_name: &syn::Ident,
    generic_scheme: Option<&GenericScheme>,
) -> proc_macro2::TokenStream {
    if let Some(generic_scheme) = generic_scheme {
        let generic_params = &generic_scheme.params;
        let generic_idents_without_const_generics = &generic_scheme.idents_without_const_generics;
        quote! {
            pub struct #interface_struct_name<#(#generic_params),*> {
                _marker: ::std::marker::PhantomData<(#(#generic_idents_without_const_generics),*)>,
            }
        }
    } else {
        quote! {
            pub struct #interface_struct_name;
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_non_generic() {
        let interface_struct_name: syn::Ident = parse_quote!(MyMethodInterface);

        let res = build_interface_struct(&interface_struct_name, None);

        let expected = quote! {
            pub struct MyMethodInterface;
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_with_single_param() {
        let interface_struct_name: syn::Ident = parse_quote!(MyMethodInterface);
        let generic_scheme = GenericScheme {
            params: vec![parse_quote!(T)],
            idents: vec![parse_quote!(T)],
            idents_without_const_generics: vec![parse_quote!(T)],
            keys: vec![],
        };

        let res = build_interface_struct(&interface_struct_name, Some(&generic_scheme));

        let expected = quote! {
            pub struct MyMethodInterface<T> {
                _marker: ::std::marker::PhantomData<(T)>,
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_with_multiple_params_excludes_const_generics_from_marker() {
        let interface_struct_name: syn::Ident = parse_quote!(MyMethodInterface);
        // Const generics can't appear inside PhantomData's tuple, so only type/lifetime idents
        // are included in `idents_without_const_generics`.
        let generic_scheme = GenericScheme {
            params: vec![parse_quote!(T), parse_quote!(const C: u32)],
            idents: vec![parse_quote!(T), parse_quote!(C)],
            idents_without_const_generics: vec![parse_quote!(T)],
            keys: vec![],
        };

        let res = build_interface_struct(&interface_struct_name, Some(&generic_scheme));

        let expected = quote! {
            pub struct MyMethodInterface<T, const C: u32> {
                _marker: ::std::marker::PhantomData<(T)>,
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_with_empty_idents_slice() {
        let interface_struct_name: syn::Ident = parse_quote!(MyMethodInterface);
        let generic_scheme = GenericScheme {
            params: vec![parse_quote!(const C: u32)],
            idents: vec![parse_quote!(C)],
            idents_without_const_generics: vec![],
            keys: vec![],
        };

        let res = build_interface_struct(&interface_struct_name, Some(&generic_scheme));

        let expected = quote! {
            pub struct MyMethodInterface<const C: u32> {
                _marker: ::std::marker::PhantomData<()>,
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }
}
