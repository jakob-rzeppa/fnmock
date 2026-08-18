use quote::quote;

pub fn build_interface_getter(
    interface_name: &syn::Ident,
    generic_params: Option<&[syn::GenericParam]>,
    generic_idents: Option<&[syn::Ident]>,
) -> proc_macro2::TokenStream {
    if let (Some(generic_params), Some(generic_idents)) = (generic_params, generic_idents) {
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
        let interface_name: syn::Ident = parse_quote!(MyMethodInterface);

        let res = build_interface_getter(&interface_name, None, None);

        let expected = quote! {
            pub(super) fn interface() -> MyMethodInterface {
                MyMethodInterface {}
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_with_single_param() {
        let interface_name: syn::Ident = parse_quote!(MyMethodInterface);
        let generic_params: Vec<syn::GenericParam> = vec![parse_quote!(T: Display + 'static)];
        let generic_idents: Vec<syn::Ident> = vec![parse_quote!(T)];

        let res = build_interface_getter(
            &interface_name,
            Some(&generic_params),
            Some(&generic_idents),
        );

        let expected = quote! {
            pub(super) fn interface<T: Display + 'static>() -> MyMethodInterface<T> {
                MyMethodInterface {
                    _marker: ::std::marker::PhantomData,
                }
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_with_multiple_params_and_const_generic() {
        let interface_name: syn::Ident = parse_quote!(MyMethodInterface);
        let generic_params: Vec<syn::GenericParam> =
            vec![parse_quote!(T), parse_quote!(const C: u32)];
        let generic_idents: Vec<syn::Ident> = vec![parse_quote!(T), parse_quote!(C)];

        let res = build_interface_getter(
            &interface_name,
            Some(&generic_params),
            Some(&generic_idents),
        );

        let expected = quote! {
            pub(super) fn interface<T, const C: u32>() -> MyMethodInterface<T, C> {
                MyMethodInterface {
                    _marker: ::std::marker::PhantomData,
                }
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }
}
