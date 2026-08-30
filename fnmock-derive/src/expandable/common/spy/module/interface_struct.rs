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
        let interface_struct_name: syn::Ident = parse_quote!(MyFunctionSpyInterface);

        let res = build_interface_struct(&interface_struct_name, None);

        let expected = quote! {
            pub struct MyFunctionSpyInterface;
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_with_single_param() {
        let interface_struct_name: syn::Ident = parse_quote!(MyFunctionSpyInterface);
        let generic_scheme = GenericScheme {
            params: vec![parse_quote!(T)],
            idents: vec![parse_quote!(T)],
            idents_without_const_generics: vec![parse_quote!(T)],
            keys: vec![],
        };

        let res = build_interface_struct(&interface_struct_name, Some(&generic_scheme));

        let expected = quote! {
            pub struct MyFunctionSpyInterface<T> {
                _marker: ::std::marker::PhantomData<(T)>,
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }
}
