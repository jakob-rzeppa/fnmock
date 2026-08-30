use syn::parse_quote;

use crate::{
    expandable::{
        common::spy::{
            inline_call::build_inline_call,
            module::{
                interface_getter::build_interface_getter, interface_impl::build_interface_impl,
                interface_struct::build_interface_struct, matcher::build_matcher,
                record_call::build_record_call, spy_store::build_spy_store,
            },
        },
        impl_block::{ImplExpandable, ImplMethodExpandable},
    },
    scheme::impl_block::{
        common::{ImplCommonMethodScheme, ImplCommonScheme},
        spy::{ImplSpyMethodScheme, ImplSpyScheme},
    },
};

impl TryFrom<ImplSpyScheme> for ImplExpandable {
    type Error = syn::Error;

    fn try_from(value: ImplSpyScheme) -> Result<Self, Self::Error> {
        let ImplSpyScheme {
            common: ImplCommonScheme { original },
            methods,
        } = value;

        let methods = methods
            .into_iter()
            .map(|(method_name, method_info)| {
                (method_name, create_impl_method_expandable(method_info))
            })
            .collect::<Vec<(syn::Ident, ImplMethodExpandable)>>();

        Ok(ImplExpandable { original, methods })
    }
}

fn create_impl_method_expandable(scheme: ImplSpyMethodScheme) -> ImplMethodExpandable {
    let ImplSpyMethodScheme {
        common:
            ImplCommonMethodScheme {
                vis,
                accessor_name,
                module_name,
                display_name,
                interface_name,
                generic_scheme,
                method_generic_params,
            },
        store_name,
        matcher_name,
        params_name,
        param_idents,
        param_types,
        params_tuple_types,
        reference_call_values,
        generic_display_fragments,
        supports_expect,
    } = scheme;

    let interface_type: syn::Type = if let Some(generic_scheme) = &generic_scheme {
        let generic_idents = &generic_scheme.idents;
        parse_quote! { #interface_name<#(#generic_idents),*> }
    } else {
        parse_quote! { #interface_name }
    };

    let matcher_type: syn::Type = if let Some(generic_scheme) = &generic_scheme {
        let generic_idents = &generic_scheme.idents;
        parse_quote! { #matcher_name<#(#generic_idents),*> }
    } else {
        parse_quote! { #matcher_name }
    };

    ImplMethodExpandable {
        vis,
        inline_call: build_inline_call(
            &module_name,
            &reference_call_values,
            generic_scheme.as_ref().map(|g| g.idents.as_slice()),
        ),
        accessor_name,
        method_generic_params,
        interface_type,
        module_name,
        module_parts: vec![
            build_spy_store(
                &store_name,
                &display_name,
                &matcher_type,
                generic_scheme.as_ref().map(|g| g.params.len()),
            ),
            build_matcher(
                &matcher_name,
                &params_name,
                &param_idents,
                &param_types,
                &params_tuple_types,
                generic_scheme.as_ref(),
                supports_expect,
            ),
            build_interface_struct(&interface_name, generic_scheme.as_ref()),
            build_interface_impl(
                &interface_name,
                &store_name,
                &matcher_name,
                &display_name,
                &param_idents,
                &param_types,
                generic_scheme.as_ref(),
                &generic_display_fragments,
                supports_expect,
            ),
            build_interface_getter(&interface_name, generic_scheme.as_ref()),
            build_record_call(
                &store_name,
                &matcher_name,
                &params_name,
                &display_name,
                &param_idents,
                &param_types,
                generic_scheme.as_ref(),
                &generic_display_fragments,
            ),
        ],
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_quote;

    use super::*;
    use crate::{
        item_info::original::OriginalImpl,
        scheme::{
            common::generic_scheme::GenericScheme,
            impl_block::{
                common::{ImplCommonMethodScheme, ImplCommonScheme},
                spy::ImplSpyMethodScheme,
            },
        },
    };

    fn non_generic_method_scheme(
        accessor_name: syn::Ident,
        module_name: syn::Ident,
        store_name: syn::Ident,
        interface_name: syn::Ident,
        matcher_name: syn::Ident,
        params_name: syn::Ident,
    ) -> ImplSpyMethodScheme {
        ImplSpyMethodScheme {
            common: ImplCommonMethodScheme {
                vis: syn::Visibility::Inherited,
                accessor_name,
                method_generic_params: vec![],
                module_name,
                display_name: "my_method".to_string(),
                interface_name,
                generic_scheme: None,
            },
            store_name,
            matcher_name,
            params_name,
            param_idents: vec![parse_quote!(a)],
            param_types: vec![parse_quote!(i32)],
            params_tuple_types: vec![parse_quote!(i32)],
            reference_call_values: vec![parse_quote!(&a)],
            generic_display_fragments: vec![],
            supports_expect: true,
        }
    }

    fn my_method_scheme() -> ImplSpyMethodScheme {
        non_generic_method_scheme(
            parse_quote!(my_method_spy),
            parse_quote!(my_method_module),
            parse_quote!(MY_METHOD_STORE),
            parse_quote!(MyMethodInterface),
            parse_quote!(MyMethodMatcher),
            parse_quote!(MyMethodMatcherParams),
        )
    }

    #[test]
    fn test_create_impl_method_expandable_non_generic() {
        let scheme = ImplSpyScheme {
            common: ImplCommonScheme {
                original: OriginalImpl::new(parse_quote! {
                    impl MyStruct {
                        fn my_method(&self, a: i32) {}
                    }
                }),
            },
            methods: vec![(parse_quote!(my_method), my_method_scheme())],
        };

        let res = ImplExpandable::try_from(scheme).unwrap();

        assert_eq!(res.methods.len(), 1);
        let method = &res.methods[0].1;

        let expected_accessor_name: syn::Ident = parse_quote!(my_method_spy);
        let expected_module_name: syn::Ident = parse_quote!(my_method_module);
        assert_eq!(method.accessor_name, expected_accessor_name);
        assert_eq!(method.module_name, expected_module_name);
        assert_eq!(
            method.interface_type.to_token_stream().to_string(),
            "MyMethodInterface"
        );
        assert_eq!(method.method_generic_params.len(), 0);
        // spy_store, matcher, interface_struct, interface_impl, interface_getter, record_call
        assert_eq!(method.module_parts.len(), 6);

        let expected_inline_call: syn::Block = parse_quote! {{
            self::my_method_module::internal_record_call(&a);
        }};
        assert_eq!(
            method.inline_call.to_token_stream().to_string(),
            expected_inline_call.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_create_impl_method_expandable_generic() {
        let mut method_scheme = my_method_scheme();
        method_scheme.common.generic_scheme = Some(GenericScheme {
            params: vec![parse_quote!(S: 'static)],
            idents: vec![parse_quote!(S)],
            idents_without_const_generics: vec![parse_quote!(S)],
            keys: vec![parse_quote!(::std::any::TypeId::of::<S>())],
        });
        let scheme = ImplSpyScheme {
            common: ImplCommonScheme {
                original: OriginalImpl::new(parse_quote! {
                    impl<S: 'static> MyStruct<S> {
                        fn my_method(&self, a: i32) {}
                    }
                }),
            },
            methods: vec![(parse_quote!(my_method), method_scheme)],
        };

        let res = ImplExpandable::try_from(scheme).unwrap();

        let method = &res.methods[0].1;
        assert_eq!(
            method.interface_type.to_token_stream().to_string(),
            "MyMethodInterface < S >"
        );

        let expected_inline_call: syn::Block = parse_quote! {{
            self::my_method_module::internal_record_call::<S>(&a);
        }};
        assert_eq!(
            method.inline_call.to_token_stream().to_string(),
            expected_inline_call.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_try_from_impl_spy_scheme_preserves_method_order_and_original() {
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

        let scheme = ImplSpyScheme {
            common: ImplCommonScheme {
                original: OriginalImpl::new(item_impl.clone()),
            },
            methods: vec![
                (
                    parse_quote!(method_one),
                    non_generic_method_scheme(
                        parse_quote!(method_one_spy),
                        parse_quote!(method_one_module),
                        parse_quote!(METHOD_ONE_STORE),
                        parse_quote!(MethodOneInterface),
                        parse_quote!(MethodOneMatcher),
                        parse_quote!(MethodOneMatcherParams),
                    ),
                ),
                (
                    parse_quote!(method_two),
                    non_generic_method_scheme(
                        parse_quote!(method_two_spy),
                        parse_quote!(method_two_module),
                        parse_quote!(METHOD_TWO_STORE),
                        parse_quote!(MethodTwoInterface),
                        parse_quote!(MethodTwoMatcher),
                        parse_quote!(MethodTwoMatcherParams),
                    ),
                ),
            ],
        };

        let res = ImplExpandable::try_from(scheme).unwrap();

        assert_eq!(
            res.original.to_token_stream().to_string(),
            item_impl.to_token_stream().to_string()
        );
        assert_eq!(res.methods.len(), 2);
        assert_eq!(res.methods[0].0.to_string(), "method_one");
        assert_eq!(res.methods[1].0.to_string(), "method_two");
        assert_eq!(res.methods[0].1.module_parts.len(), 6);
        assert_eq!(res.methods[1].1.module_parts.len(), 6);
    }
}
