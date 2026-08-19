use syn::parse_quote;

pub struct AccessorMethodInfo<'a> {
    pub vis: &'a syn::Visibility,
    pub name: &'a syn::Ident,
    pub method_generic_params: &'a [syn::GenericParam],
    pub module_name: &'a syn::Ident,
    pub interface_type: &'a syn::Type,
}

pub fn build_accessor_impl(
    struct_name: &syn::Ident,
    struct_generic_params: &[syn::GenericParam],
    struct_generic_idents: &[syn::Ident],
    methods: &[AccessorMethodInfo<'_>],
) -> syn::ItemImpl {
    let methods = methods
        .iter()
        .map(
            |AccessorMethodInfo {
                 vis,
                 name,
                 method_generic_params,
                 module_name,
                 interface_type,
             }| {
                build_accessor_method(
                    vis,
                    name,
                    method_generic_params,
                    module_name,
                    interface_type,
                )
            },
        )
        .collect::<Vec<syn::ItemFn>>();

    let self_ty: syn::Type = if struct_generic_idents.is_empty() {
        parse_quote!(#struct_name)
    } else {
        parse_quote!(#struct_name<#(#struct_generic_idents),*>)
    };

    parse_quote! {
        #[cfg(test)]
        impl<#(#struct_generic_params),*> #self_ty {
            #(#methods)*
        }
    }
}

fn build_accessor_method(
    vis: &syn::Visibility,
    name: &syn::Ident,
    method_generic_params: &[syn::GenericParam],
    module_name: &syn::Ident,
    interface_type: &syn::Type,
) -> syn::ItemFn {
    parse_quote! {
        #vis fn #name<#(#method_generic_params),*>() -> self::#module_name::#interface_type {
            self::#module_name::interface()
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    #[test]
    fn test_build_accessor_impl_no_methods() {
        let struct_name: syn::Ident = parse_quote!(MyStruct);
        let struct_generic_params: Vec<syn::GenericParam> = vec![];
        let struct_generic_idents: Vec<syn::Ident> = vec![];
        let methods: Vec<AccessorMethodInfo<'_>> = vec![];

        let res = build_accessor_impl(
            &struct_name,
            &struct_generic_params,
            &struct_generic_idents,
            &methods,
        );

        let expected: syn::ItemImpl = parse_quote! {
            impl MyStruct {}
        };

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_build_accessor_impl_single_method_no_generics() {
        let struct_name: syn::Ident = parse_quote!(MyStruct);
        let struct_generic_params: Vec<syn::GenericParam> = vec![];
        let struct_generic_idents: Vec<syn::Ident> = vec![];

        let vis: syn::Visibility = parse_quote!(pub);
        let name: syn::Ident = parse_quote!(my_method_fake);
        let method_generic_params: Vec<syn::GenericParam> = vec![];
        let module_name: syn::Ident = parse_quote!(my_method_module);
        let interface_type: syn::Type = parse_quote!(FakeInterface);

        let methods = vec![AccessorMethodInfo {
            vis: &vis,
            name: &name,
            method_generic_params: &method_generic_params,
            module_name: &module_name,
            interface_type: &interface_type,
        }];

        let res = build_accessor_impl(
            &struct_name,
            &struct_generic_params,
            &struct_generic_idents,
            &methods,
        );

        let expected: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                pub fn my_method_fake() -> self::my_method_module::FakeInterface {
                    self::my_method_module::interface()
                }
            }
        };

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_build_accessor_impl_with_struct_and_method_generics() {
        let struct_name: syn::Ident = parse_quote!(MyStruct);
        let struct_generic_params: Vec<syn::GenericParam> = vec![parse_quote!(S)];
        let struct_generic_idents: Vec<syn::Ident> = vec![parse_quote!(S)];

        let vis: syn::Visibility = parse_quote!(pub);
        let name: syn::Ident = parse_quote!(my_method_fake);
        let method_generic_params: Vec<syn::GenericParam> = vec![parse_quote!(T)];
        let module_name: syn::Ident = parse_quote!(my_method_module);
        let interface_type: syn::Type = parse_quote!(FakeInterface<S, T>);

        let methods = vec![AccessorMethodInfo {
            vis: &vis,
            name: &name,
            method_generic_params: &method_generic_params,
            module_name: &module_name,
            interface_type: &interface_type,
        }];

        let res = build_accessor_impl(
            &struct_name,
            &struct_generic_params,
            &struct_generic_idents,
            &methods,
        );

        let expected: syn::ItemImpl = parse_quote! {
            impl<S> MyStruct<S> {
                pub fn my_method_fake<T>() -> self::my_method_module::FakeInterface<S, T> {
                    self::my_method_module::interface()
                }
            }
        };

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_build_accessor_impl_multiple_methods() {
        let struct_name: syn::Ident = parse_quote!(MyStruct);
        let struct_generic_params: Vec<syn::GenericParam> = vec![];
        let struct_generic_idents: Vec<syn::Ident> = vec![];

        let vis_a: syn::Visibility = parse_quote!(pub);
        let name_a: syn::Ident = parse_quote!(method_a_fake);
        let method_generic_params_a: Vec<syn::GenericParam> = vec![];
        let module_name_a: syn::Ident = parse_quote!(method_a_module);
        let interface_type_a: syn::Type = parse_quote!(FakeInterfaceA);

        let vis_b: syn::Visibility = syn::Visibility::Inherited;
        let name_b: syn::Ident = parse_quote!(method_b_fake);
        let method_generic_params_b: Vec<syn::GenericParam> = vec![];
        let module_name_b: syn::Ident = parse_quote!(method_b_module);
        let interface_type_b: syn::Type = parse_quote!(FakeInterfaceB);

        let methods = vec![
            AccessorMethodInfo {
                vis: &vis_a,
                name: &name_a,
                method_generic_params: &method_generic_params_a,
                module_name: &module_name_a,
                interface_type: &interface_type_a,
            },
            AccessorMethodInfo {
                vis: &vis_b,
                name: &name_b,
                method_generic_params: &method_generic_params_b,
                module_name: &module_name_b,
                interface_type: &interface_type_b,
            },
        ];

        let res = build_accessor_impl(
            &struct_name,
            &struct_generic_params,
            &struct_generic_idents,
            &methods,
        );

        let expected: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                pub fn method_a_fake() -> self::method_a_module::FakeInterfaceA {
                    self::method_a_module::interface()
                }
                fn method_b_fake() -> self::method_b_module::FakeInterfaceB {
                    self::method_b_module::interface()
                }
            }
        };

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
}
