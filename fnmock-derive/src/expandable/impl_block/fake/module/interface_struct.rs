use quote::quote;

pub fn build_interface_struct(
    interface_struct_name: &syn::Ident,
    generic_params: Option<&[syn::GenericParam]>,
    generic_idents_without_const_generics: Option<&[syn::Ident]>,
) -> proc_macro2::TokenStream {
    if let (Some(generic_params), Some(generic_idents_without_const_generics)) =
        (generic_params, generic_idents_without_const_generics)
    {
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

        let res = build_interface_struct(&interface_struct_name, None, None);

        let expected = quote! {
            pub struct MyMethodInterface;
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_with_single_param() {
        let interface_struct_name: syn::Ident = parse_quote!(MyMethodInterface);
        let generic_params: Vec<syn::GenericParam> = vec![parse_quote!(T)];
        let generic_idents_without_const_generics: Vec<syn::Ident> = vec![parse_quote!(T)];

        let res = build_interface_struct(
            &interface_struct_name,
            Some(&generic_params),
            Some(&generic_idents_without_const_generics),
        );

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
        let generic_params: Vec<syn::GenericParam> =
            vec![parse_quote!(T), parse_quote!(const C: u32)];
        // Const generics can't appear inside PhantomData's tuple, so only type/lifetime idents
        // are passed here.
        let generic_idents_without_const_generics: Vec<syn::Ident> = vec![parse_quote!(T)];

        let res = build_interface_struct(
            &interface_struct_name,
            Some(&generic_params),
            Some(&generic_idents_without_const_generics),
        );

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
        let generic_params: Vec<syn::GenericParam> = vec![parse_quote!(const C: u32)];
        let generic_idents_without_const_generics: Vec<syn::Ident> = vec![];

        let res = build_interface_struct(
            &interface_struct_name,
            Some(&generic_params),
            Some(&generic_idents_without_const_generics),
        );

        let expected = quote! {
            pub struct MyMethodInterface<const C: u32> {
                _marker: ::std::marker::PhantomData<()>,
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_only_generic_params_without_idents_falls_back_to_unit_struct() {
        let interface_struct_name: syn::Ident = parse_quote!(MyMethodInterface);
        let generic_params: Vec<syn::GenericParam> = vec![parse_quote!(T)];

        let res = build_interface_struct(&interface_struct_name, Some(&generic_params), None);

        let expected = quote! {
            pub struct MyMethodInterface;
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_only_generic_idents_without_params_falls_back_to_unit_struct() {
        let interface_struct_name: syn::Ident = parse_quote!(MyMethodInterface);
        let generic_idents_without_const_generics: Vec<syn::Ident> = vec![parse_quote!(T)];

        let res = build_interface_struct(
            &interface_struct_name,
            None,
            Some(&generic_idents_without_const_generics),
        );

        let expected = quote! {
            pub struct MyMethodInterface;
        };

        assert_eq!(res.to_string(), expected.to_string());
    }
}
