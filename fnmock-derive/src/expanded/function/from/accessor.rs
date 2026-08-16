use syn::parse_quote;

pub fn build_accessor(
    vis: &syn::Visibility,
    name: &syn::Ident,
    generic_params: &[syn::GenericParam],
    interface_getter: &syn::Expr,
    interface_type: &syn::Type,
) -> syn::ItemFn {
    parse_quote! (
        #[cfg(test)]
        #vis fn #name<#(#generic_params),*>() -> #interface_type {
            #interface_getter
        }
    )
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    #[test]
    fn test_build_accessor() {
        let vis: syn::Visibility = parse_quote!(pub);
        let name: syn::Ident = parse_quote!(my_function_fake);
        let generic_params: Vec<syn::GenericParam> = vec![];
        let interface_getter: syn::Expr = parse_quote!(self::some_module::interface());
        let interface_type: syn::Type = parse_quote!(SomeInterface);

        let res = build_accessor(
            &vis,
            &name,
            &generic_params,
            &interface_getter,
            &interface_type,
        );

        let expected: syn::ItemFn = parse_quote! {
            #[cfg(test)]
            pub fn my_function_fake() -> SomeInterface {
                self::some_module::interface()
            }
        };

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_build_private_visibility() {
        let vis: syn::Visibility = syn::Visibility::Inherited;
        let name: syn::Ident = parse_quote!(my_function_fake);
        let generic_params: Vec<syn::GenericParam> = vec![];
        let interface_getter: syn::Expr = parse_quote!(self::some_module::interface());
        let interface_type: syn::Type = parse_quote!(SomeInterface);

        let res = build_accessor(
            &vis,
            &name,
            &generic_params,
            &interface_getter,
            &interface_type,
        );

        let expected: syn::ItemFn = parse_quote! {
            #[cfg(test)]
            fn my_function_fake() -> SomeInterface {
                self::some_module::interface()
            }
        };

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_build_with_generic_params() {
        let vis: syn::Visibility = parse_quote!(pub);
        let name: syn::Ident = parse_quote!(my_function_fake);
        let generic_params: Vec<syn::GenericParam> =
            vec![parse_quote!(T), parse_quote!(const C: u32)];
        let interface_getter: syn::Expr =
            parse_quote!(self::my_function_module::interface::<T, C>());
        let interface_type: syn::Type = parse_quote!(FakeInterface<T, C>);

        let res = build_accessor(
            &vis,
            &name,
            &generic_params,
            &interface_getter,
            &interface_type,
        );

        let expected: syn::ItemFn = parse_quote! {
            #[cfg(test)]
            pub fn my_function_fake<T, const C: u32>() -> FakeInterface<T, C> {
                self::my_function_module::interface::<T, C>()
            }
        };

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_build_with_generic_bounds() {
        let vis: syn::Visibility = parse_quote!(pub);
        let name: syn::Ident = parse_quote!(my_function_fake);
        let generic_params: Vec<syn::GenericParam> = vec![parse_quote!(T: Clone)];
        let interface_getter: syn::Expr = parse_quote!(self::my_function_module::interface::<T>());
        let interface_type: syn::Type = parse_quote!(FakeInterface<T>);

        let res = build_accessor(
            &vis,
            &name,
            &generic_params,
            &interface_getter,
            &interface_type,
        );

        let expected: syn::ItemFn = parse_quote! {
            #[cfg(test)]
            pub fn my_function_fake<T: Clone>() -> FakeInterface<T> {
                self::my_function_module::interface::<T>()
            }
        };

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_build_with_generic_with_lifetime_bound() {
        let vis: syn::Visibility = parse_quote!(pub);
        let name: syn::Ident = parse_quote!(my_function_fake);
        let generic_params: Vec<syn::GenericParam> = vec![parse_quote!(T: 'static)];
        let interface_getter: syn::Expr = parse_quote!(self::my_function_module::interface::<T>());
        let interface_type: syn::Type = parse_quote!(FakeInterface<T>);

        let res = build_accessor(
            &vis,
            &name,
            &generic_params,
            &interface_getter,
            &interface_type,
        );

        let expected: syn::ItemFn = parse_quote! {
            #[cfg(test)]
            pub fn my_function_fake<T: 'static>() -> FakeInterface<T> {
                self::my_function_module::interface::<T>()
            }
        };

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
}
