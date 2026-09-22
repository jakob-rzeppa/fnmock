use crate::{
    item_info::{
        generics::GenericParamInfo,
        impl_block::{ImplBlockInfo, ImplMethodInfo},
        param_info::ParamInfo,
    },
    scheme::{
        common::{
            generic_scheme::{build_generic_display_fragment, build_generic_scheme},
            spy_param::build_spy_params,
        },
        function::spy::SpyScheme,
        impl_block::{
            common::{ImplCommonMethodScheme, ImplCommonScheme, combine_generic_param_infos},
            spy::names::{
                build_accessor_name, build_interface_name, build_matcher_name, build_module_name,
                build_params_name, build_store_name,
            },
        },
    },
};

mod names;

pub struct ImplSpyScheme {
    pub common: ImplCommonScheme,

    /// The order of the methods must be preserved from the original impl block.
    /// (method_name, method_info)
    pub methods: Vec<(syn::Ident, ImplSpyMethodScheme)>,
}

pub struct ImplSpyMethodScheme {
    pub common: ImplCommonMethodScheme,

    pub spy: SpyScheme,
}

impl TryFrom<ImplBlockInfo> for ImplSpyScheme {
    type Error = syn::Error;

    fn try_from(value: ImplBlockInfo) -> Result<Self, Self::Error> {
        let ImplBlockInfo {
            original,
            struct_name,
            generic_param_infos,
            functions,
        } = value;

        let methods = functions
            .into_iter()
            .map(|method| build_method_scheme(&struct_name, &generic_param_infos, method))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ImplSpyScheme {
            common: ImplCommonScheme { original },
            methods,
        })
    }
}

pub fn build_spy_scheme(
    struct_name: &syn::TypePath,
    method_name: &syn::Ident,
    param_infos: &[ParamInfo],
    combined_generic_param_infos: &[GenericParamInfo],
) -> syn::Result<SpyScheme> {
    let store_name = build_store_name(struct_name, method_name);
    let matcher_name = build_matcher_name(struct_name, method_name)?;
    let params_name = build_params_name(struct_name, method_name)?;

    let params = build_spy_params(param_infos)?;

    let generic_display_fragments = combined_generic_param_infos
        .iter()
        .map(build_generic_display_fragment)
        .collect();

    Ok(SpyScheme {
        store_name,
        matcher_name,
        params_name,
        param_idents: params.idents,
        param_types: params.types,
        params_tuple_types: params.params_tuple_types,
        reference_call_values: params.reference_call_values,
        generic_display_fragments,
        supports_expect: params.supports_expect,
    })
}

/// Builds the spy scheme for a single method, merging the struct's generics (shared by every
/// method) with the method's own.
fn build_method_scheme(
    struct_name: &syn::TypePath,
    struct_generic_param_infos: &[GenericParamInfo],
    method: ImplMethodInfo,
) -> syn::Result<(syn::Ident, ImplSpyMethodScheme)> {
    let ImplMethodInfo {
        method_name,
        visibility,
        param_infos,
        lifetimes: _,
        return_type: _,
        generic_param_infos: method_generic_param_infos,
    } = method;

    let (method_generic_params, combined_generic_param_infos) =
        combine_generic_param_infos(struct_generic_param_infos, method_generic_param_infos);

    let spy = build_spy_scheme(
        struct_name,
        &method_name,
        &param_infos,
        &combined_generic_param_infos,
    )?;

    let module_name = build_module_name(struct_name, &method_name);
    let accessor_name = build_accessor_name(&method_name);
    let interface_name = build_interface_name(struct_name, &method_name)?;
    let display_name = method_name.to_string();

    let generic_scheme = build_generic_scheme(&combined_generic_param_infos);

    Ok((
        method_name,
        ImplSpyMethodScheme {
            common: ImplCommonMethodScheme {
                vis: visibility,
                accessor_name,
                module_name,
                display_name,
                interface_name,
                generic_scheme,
                method_generic_params,
            },
            spy,
        },
    ))
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_non_generic_single_method_impl_block() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl UserService {
                fn get_user(&self, id: u32) -> String {
                    todo!()
                }
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let scheme = ImplSpyScheme::try_from(info)
            .expect("conversion should succeed for a non-generic impl block");

        assert_eq!(scheme.methods.len(), 1);
        let method = &scheme.methods[0];
        assert_eq!(method.0.to_string(), "get_user");
        assert_eq!(
            method.1.common.module_name.to_string(),
            "user_service__get_user_spy_module"
        );
        assert_eq!(method.1.common.display_name, "get_user");
        assert_eq!(method.1.common.accessor_name.to_string(), "get_user_spy");
        assert!(method.1.common.method_generic_params.is_empty());
        assert!(method.1.common.generic_scheme.is_none());
        assert_eq!(
            method.1.common.interface_name.to_string(),
            "UserServiceGetUserSpyInterface"
        );
        assert_eq!(
            method.1.spy.store_name.to_string(),
            "USER_SERVICE_GET_USER_SPY_STORE"
        );
        assert_eq!(
            method.1.spy.matcher_name.to_string(),
            "UserServiceGetUserMatcher"
        );
        assert_eq!(
            method.1.spy.params_name.to_string(),
            "UserServiceGetUserMatcherParams"
        );
        assert!(method.1.spy.supports_expect);
    }

    #[test]
    fn test_receiver_is_not_recorded() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl UserService {
                fn add(&self, a: i32, b: String) -> i32 {
                    todo!()
                }
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let scheme = ImplSpyScheme::try_from(info).expect("conversion should succeed");

        let method = &scheme.methods[0];
        assert_eq!(
            method
                .1
                .spy
                .param_idents
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>(),
            vec!["a".to_string(), "b".to_string()]
        );
        assert_eq!(
            method
                .1
                .spy
                .param_types
                .iter()
                .map(|t| t.to_token_stream().to_string())
                .collect::<Vec<_>>(),
            vec!["i32".to_string(), "String".to_string()]
        );
        assert_eq!(
            method
                .1
                .spy
                .reference_call_values
                .iter()
                .map(|v| v.to_token_stream().to_string())
                .collect::<Vec<_>>(),
            vec!["& a".to_string(), "& b".to_string()]
        );
    }

    #[test]
    fn test_consuming_receiver_is_not_recorded_either() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl UserService {
                fn consume(self, tag: u8) {}
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let scheme = ImplSpyScheme::try_from(info).expect("conversion should succeed");

        let method = &scheme.methods[0];
        assert_eq!(
            method
                .1
                .spy
                .param_idents
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>(),
            vec!["tag".to_string()]
        );
    }

    #[test]
    fn test_associated_function_records_every_param() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl UserService {
                fn create(name: String) -> UserService {
                    todo!()
                }
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let scheme = ImplSpyScheme::try_from(info).expect("conversion should succeed");

        let method = &scheme.methods[0];
        assert_eq!(
            method
                .1
                .spy
                .param_idents
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>(),
            vec!["name".to_string()]
        );
    }

    #[test]
    fn test_multiple_methods_get_distinct_non_colliding_names() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl UserService {
                fn get_user(&self) -> i32 { 42 }
                pub fn save_user(&self) {}
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let scheme = ImplSpyScheme::try_from(info).expect("conversion should succeed");

        assert_eq!(scheme.methods.len(), 2);
        assert_eq!(scheme.methods[0].0.to_string(), "get_user");
        assert_eq!(scheme.methods[1].0.to_string(), "save_user");
        assert_ne!(
            scheme.methods[0].1.common.module_name,
            scheme.methods[1].1.common.module_name
        );
        assert_ne!(
            scheme.methods[0].1.spy.store_name,
            scheme.methods[1].1.spy.store_name
        );
    }

    #[test]
    fn test_struct_and_method_generics_are_combined_struct_then_method() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl<S: 'static> Foo<S> {
                fn bar<M: 'static>(&self, x: S, y: M) {}
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let scheme = ImplSpyScheme::try_from(info).expect("conversion should succeed");

        let method = &scheme.methods[0];
        let generic_scheme = method
            .1
            .common
            .generic_scheme
            .as_ref()
            .expect("expected generic_scheme to be Some");
        assert_eq!(generic_scheme.params.len(), 2);
        assert_eq!(
            generic_scheme
                .idents
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>(),
            vec!["S".to_string(), "M".to_string()]
        );
        // Only the method's own generics get redeclared on the accessor; the struct's are already
        // in scope from the enclosing `impl<..>` block.
        assert_eq!(method.1.common.method_generic_params.len(), 1);
        assert_eq!(method.1.spy.generic_display_fragments.len(), 2);
        assert_eq!(
            method.1.spy.generic_display_fragments[0]
                .to_token_stream()
                .to_string(),
            quote::quote!(::std::any::type_name::<S>().to_string()).to_string()
        );
    }

    #[test]
    fn test_const_generic_display_fragment_uses_the_value_not_the_type() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl Foo {
                fn bar<const N: usize>(&self) -> usize { N }
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let scheme = ImplSpyScheme::try_from(info).expect("conversion should succeed");

        let method = &scheme.methods[0];
        assert_eq!(
            method.1.spy.generic_display_fragments[0]
                .to_token_stream()
                .to_string(),
            quote::quote!(N.to_string()).to_string()
        );
    }

    #[test]
    fn test_lifetime_bearing_param_type_disables_expect() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl Foo {
                fn bar(&self, r: Ref<'_>) -> usize {
                    todo!()
                }
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let scheme = ImplSpyScheme::try_from(info).expect("conversion should succeed");

        assert!(!scheme.methods[0].1.spy.supports_expect);
    }

    #[test]
    fn test_lifetime_bearing_receiver_does_not_disable_expect() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl Foo {
                fn bar<'a>(&'a self, id: u32) -> usize {
                    todo!()
                }
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let scheme = ImplSpyScheme::try_from(info).expect("conversion should succeed");

        assert!(scheme.methods[0].1.spy.supports_expect);
    }

    #[test]
    fn test_destructuring_param_is_rejected() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl Foo {
                fn bar(&self, (a, b): (i32, i32)) {}
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let result = ImplSpyScheme::try_from(info);

        let Err(error) = result else {
            panic!(
                "a destructuring parameter should be rejected: a matcher needs one name per parameter"
            );
        };
        assert!(error.to_string().contains("#[spyable]"));
    }

    #[test]
    fn test_wildcard_param_is_rejected() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl Foo {
                fn bar(&self, _: u32) {}
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        assert!(ImplSpyScheme::try_from(info).is_err());
    }

    #[test]
    fn test_impl_trait_param_is_rejected() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl Foo {
                fn bar(&self, x: impl Clone) {}
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        assert!(ImplSpyScheme::try_from(info).is_err());
    }
}
