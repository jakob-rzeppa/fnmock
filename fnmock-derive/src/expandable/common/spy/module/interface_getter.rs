use quote::quote;

use crate::scheme::common::generic_scheme::GenericScheme;

pub fn build_interface_getter(
    interface_name: &syn::Ident,
    generic_scheme: Option<&GenericScheme>,
) -> proc_macro2::TokenStream {
    if let Some(generic_scheme) = generic_scheme {
        let generic_params = &generic_scheme.params;
        let generic_idents = &generic_scheme.idents;
        quote! {
            pub(super) fn interface<#(#generic_params),*>()
                -> #interface_name<#(#generic_idents),*>
            {
                #interface_name {
                    _marker: ::std::marker::PhantomData,
                }
            }
        }
    } else {
        quote! {
            pub(super) fn interface() -> #interface_name {
                #interface_name {}
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
        let interface_name: syn::Ident = parse_quote!(MyFunctionSpyInterface);

        let res = build_interface_getter(&interface_name, None);

        let expected = quote! {
            pub(super) fn interface() -> MyFunctionSpyInterface {
                MyFunctionSpyInterface {}
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_with_single_param() {
        let interface_name: syn::Ident = parse_quote!(MyFunctionSpyInterface);
        let generic_scheme = GenericScheme {
            params: vec![parse_quote!(T: 'static)],
            idents: vec![parse_quote!(T)],
            idents_without_const_generics: vec![parse_quote!(T)],
            keys: vec![],
        };

        let res = build_interface_getter(&interface_name, Some(&generic_scheme));

        let expected = quote! {
            pub(super) fn interface<T: 'static>() -> MyFunctionSpyInterface<T> {
                MyFunctionSpyInterface {
                    _marker: ::std::marker::PhantomData,
                }
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }
}
