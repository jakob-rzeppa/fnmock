use crate::{
    expandable::impl_block::{
        ImplExpandable, ImplMethodExpandable,
        fake::{
            inline_call::build_inline_call,
            module::{
                fake_store::build_fake_store, implementation_getter::build_implementation_getter,
                interface_getter::build_interface_getter, interface_impl::build_interface_impl,
                interface_struct::build_interface_struct,
            },
        },
    },
    scheme::impl_block::{
        common::{ImplCommonMethodScheme, ImplCommonScheme},
        fake::{ImplFakeMethodScheme, ImplFakeScheme},
    },
};

mod inline_call;
mod module {
    pub mod fake_store;
    pub mod implementation_getter;
    pub mod interface_getter;
    pub mod interface_impl;
    pub mod interface_struct;
}

impl TryFrom<ImplFakeScheme> for ImplExpandable {
    type Error = syn::Error;

    fn try_from(value: ImplFakeScheme) -> Result<Self, Self::Error> {
        let ImplFakeScheme {
            common: ImplCommonScheme { original },
            methods,
        } = value;

        let methods = methods
            .into_iter()
            .map(|method| {
                let method_name = method.common.method_name.clone();
                (method_name, create_impl_method_expandable(method))
            })
            .collect::<Vec<(syn::Ident, ImplMethodExpandable)>>();

        Ok(ImplExpandable { original, methods })
    }
}

fn create_impl_method_expandable(scheme: ImplFakeMethodScheme) -> ImplMethodExpandable {
    let ImplFakeMethodScheme {
        common:
            ImplCommonMethodScheme {
                vis,
                method_name: _,
                accessor_name,
                method_generic_params,
                module_name,
                display_name,
            },
        store_name,
        fn_closure_trait,
        interface_name,
        interface_type,
        fake_call_values,
        generic_scheme,
    } = scheme;

    ImplMethodExpandable {
        vis,
        inline_call: build_inline_call(
            &module_name,
            &fake_call_values,
            generic_scheme.as_ref().map(|g| g.idents.as_slice()),
        ),
        accessor_name,
        method_generic_params,
        interface_type,
        module_name,
        module_parts: vec![
            build_fake_store(
                &store_name,
                &display_name,
                &fn_closure_trait,
                generic_scheme.as_ref().map(|g| g.params.len()),
            ),
            build_implementation_getter(&store_name, &fn_closure_trait, generic_scheme.as_ref()),
            build_interface_struct(&interface_name, generic_scheme.as_ref()),
            build_interface_impl(
                &interface_name,
                &store_name,
                generic_scheme.as_ref(),
                &fn_closure_trait,
            ),
            build_interface_getter(&interface_name, generic_scheme.as_ref()),
        ],
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_quote;

    use super::*;
    use crate::{item_info::original::OriginalImpl, scheme::common::generic_scheme::GenericScheme};

    fn non_generic_method_scheme(
        method_name: syn::Ident,
        accessor_name: syn::Ident,
        module_name: syn::Ident,
        store_name: syn::Ident,
        interface_name: syn::Ident,
    ) -> ImplFakeMethodScheme {
        ImplFakeMethodScheme {
            common: ImplCommonMethodScheme {
                vis: syn::Visibility::Inherited,
                method_name,
                accessor_name,
                method_generic_params: vec![],
                module_name,
                display_name: "my_method".to_string(),
            },
            store_name,
            fn_closure_trait: parse_quote!(Fn() -> i32),
            interface_name: interface_name.clone(),
            interface_type: syn::Type::Path(syn::TypePath {
                qself: None,
                path: interface_name.into(),
            }),
            fake_call_values: vec![],
            generic_scheme: None,
        }
    }

    #[test]
    fn test_create_impl_method_expandable_non_generic() {
        let scheme = non_generic_method_scheme(
            parse_quote!(my_method),
            parse_quote!(my_method_fake),
            parse_quote!(my_method_module),
            parse_quote!(MY_METHOD_STORE),
            parse_quote!(MyMethodInterface),
        );

        let res = create_impl_method_expandable(scheme);

        let expected_accessor_name: syn::Ident = parse_quote!(my_method_fake);
        let expected_module_name: syn::Ident = parse_quote!(my_method_module);
        assert_eq!(res.accessor_name, expected_accessor_name);
        assert_eq!(res.module_name, expected_module_name);
        assert_eq!(
            res.interface_type.to_token_stream().to_string(),
            "MyMethodInterface"
        );
        assert_eq!(res.method_generic_params.len(), 0);
        assert_eq!(res.module_parts.len(), 5);

        let expected_inline_call: syn::Block = parse_quote! {{
            if let Some(implementation) = self::my_method_module::implementation() {
                return implementation();
            }
        }};
        assert_eq!(
            res.inline_call.to_token_stream().to_string(),
            expected_inline_call.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_create_impl_method_expandable_generic() {
        let mut scheme = non_generic_method_scheme(
            parse_quote!(my_method),
            parse_quote!(my_method_fake),
            parse_quote!(my_method_module),
            parse_quote!(MY_METHOD_STORE),
            parse_quote!(MyMethodInterface),
        );
        scheme.interface_type = parse_quote!(MyMethodInterface<S>);
        scheme.generic_scheme = Some(GenericScheme {
            params: vec![parse_quote!(S: 'static)],
            idents: vec![parse_quote!(S)],
            idents_without_const_generics: vec![parse_quote!(S)],
            keys: vec![parse_quote!(::std::any::TypeId::of::<S>())],
        });

        let res = create_impl_method_expandable(scheme);

        assert_eq!(
            res.interface_type.to_token_stream().to_string(),
            "MyMethodInterface < S >"
        );

        let expected_inline_call: syn::Block = parse_quote! {{
            if let Some(implementation) = self::my_method_module::implementation::<S>() {
                return implementation();
            }
        }};
        assert_eq!(
            res.inline_call.to_token_stream().to_string(),
            expected_inline_call.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_try_from_impl_fake_scheme_multiple_methods() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                fn method_one(&self) -> i32 {
                    1
                }

                pub fn method_two(&self) -> i32 {
                    2
                }
            }
        };

        let scheme = ImplFakeScheme {
            common: ImplCommonScheme {
                original: OriginalImpl::new(item_impl.clone()),
            },
            methods: vec![
                non_generic_method_scheme(
                    parse_quote!(method_one),
                    parse_quote!(method_one_fake),
                    parse_quote!(method_one_module),
                    parse_quote!(METHOD_ONE_STORE),
                    parse_quote!(MethodOneInterface),
                ),
                non_generic_method_scheme(
                    parse_quote!(method_two),
                    parse_quote!(method_two_fake),
                    parse_quote!(method_two_module),
                    parse_quote!(METHOD_TWO_STORE),
                    parse_quote!(MethodTwoInterface),
                ),
            ],
        };

        let res = ImplExpandable::try_from(scheme).unwrap();

        assert_eq!(
            res.original.to_token_stream().to_string(),
            item_impl.to_token_stream().to_string()
        );
        assert_eq!(res.methods.len(), 2);

        let expected_method_one_name: syn::Ident = parse_quote!(method_one);
        let expected_method_one_accessor: syn::Ident = parse_quote!(method_one_fake);
        let expected_method_one_module: syn::Ident = parse_quote!(method_one_module);
        let expected_method_one_inline_call: syn::Block = parse_quote! {{
            if let Some(implementation) = self::method_one_module::implementation() {
                return implementation();
            }
        }};
        let method_one = &res.methods[0];
        assert_eq!(method_one.0, expected_method_one_name);
        assert!(matches!(method_one.1.vis, syn::Visibility::Inherited));
        assert_eq!(
            method_one.1.inline_call.to_token_stream().to_string(),
            expected_method_one_inline_call
                .to_token_stream()
                .to_string()
        );
        assert_eq!(method_one.1.accessor_name, expected_method_one_accessor);
        assert_eq!(method_one.1.method_generic_params.len(), 0);
        assert_eq!(
            method_one.1.interface_type.to_token_stream().to_string(),
            "MethodOneInterface"
        );
        assert_eq!(method_one.1.module_name, expected_method_one_module);
        assert_eq!(method_one.1.module_parts.len(), 5);

        let expected_method_two_name: syn::Ident = parse_quote!(method_two);
        let expected_method_two_accessor: syn::Ident = parse_quote!(method_two_fake);
        let expected_method_two_module: syn::Ident = parse_quote!(method_two_module);
        let expected_method_two_inline_call: syn::Block = parse_quote! {{
            if let Some(implementation) = self::method_two_module::implementation() {
                return implementation();
            }
        }};
        let method_two = &res.methods[1];
        assert_eq!(method_two.0, expected_method_two_name);
        assert!(matches!(method_two.1.vis, syn::Visibility::Inherited));
        assert_eq!(
            method_two.1.inline_call.to_token_stream().to_string(),
            expected_method_two_inline_call
                .to_token_stream()
                .to_string()
        );
        assert_eq!(method_two.1.accessor_name, expected_method_two_accessor);
        assert_eq!(method_two.1.method_generic_params.len(), 0);
        assert_eq!(
            method_two.1.interface_type.to_token_stream().to_string(),
            "MethodTwoInterface"
        );
        assert_eq!(method_two.1.module_name, expected_method_two_module);
        assert_eq!(method_two.1.module_parts.len(), 5);
    }
}
