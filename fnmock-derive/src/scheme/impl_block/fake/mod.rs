use syn::parse_quote;

use crate::{
    item_info::{
        call_value::CallValue,
        generic_param_info::GenericParamInfo,
        impl_block::{ImplBlockInfo, ImplMethodInfo},
    },
    scheme::{
        common::{
            fn_closure_trait::build_fn_closure_trait,
            generic_scheme::{GenericScheme, build_generic_scheme},
        },
        impl_block::{
            common::{ImplCommonMethodScheme, ImplCommonScheme},
            fake::names::{
                build_accessor_name, build_interface_name, build_module_name, build_store_name,
            },
        },
    },
};

mod names;

pub struct ImplFakeScheme {
    pub common: ImplCommonScheme,

    /// The order of the methods must be preserved from the original impl block.
    pub methods: Vec<ImplFakeMethodScheme>,
}

pub struct ImplFakeMethodScheme {
    pub common: ImplCommonMethodScheme,

    pub store_name: syn::Ident,

    pub fn_closure_trait: syn::TraitBound,

    pub interface_name: syn::Ident,
    pub interface_type: syn::Type,
    pub fake_call_values: Vec<CallValue>,

    /// The struct's and method's generics, combined.
    pub generic_scheme: Option<GenericScheme>,
}

impl TryFrom<ImplBlockInfo> for ImplFakeScheme {
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

        Ok(ImplFakeScheme {
            common: ImplCommonScheme { original },
            methods,
        })
    }
}

/// Builds the fake scheme for a single method, merging the struct's generics (shared by every
/// method) with the method's own.
fn build_method_scheme(
    struct_name: &syn::TypePath,
    struct_generic_param_infos: &[GenericParamInfo],
    method: ImplMethodInfo,
) -> syn::Result<ImplFakeMethodScheme> {
    let ImplMethodInfo {
        method_name,
        visibility,
        param_infos,
        lifetimes,
        return_type,
        generic_param_infos: method_generic_param_infos,
    } = method;

    let module_name = build_module_name(struct_name, &method_name);
    let store_name = build_store_name(struct_name, &method_name);
    let accessor_name = build_accessor_name(&method_name);
    let interface_name = build_interface_name(struct_name, &method_name)?;
    let display_name = method_name.to_string();

    let param_types = param_infos.iter().map(|p| p.ty.clone()).collect::<Vec<_>>();
    let fn_closure_trait = build_fn_closure_trait(&lifetimes, &param_types, &return_type)?;

    let fake_call_values = param_infos
        .iter()
        .map(|p| CallValue::try_from(&p.pat))
        .collect::<Result<Vec<_>, _>>()?;

    let method_generic_params = method_generic_param_infos
        .iter()
        .map(|g| g.param.clone())
        .collect::<Vec<_>>();

    let mut combined_generic_param_infos = struct_generic_param_infos.to_vec();
    for method_generic_param_info in method_generic_param_infos {
        combined_generic_param_infos.push(method_generic_param_info);
    }

    let generic_scheme = build_generic_scheme(&combined_generic_param_infos);

    let interface_type: syn::Type = if let Some(generic_scheme) = &generic_scheme {
        let generic_idents = &generic_scheme.idents;
        parse_quote! { #interface_name<#(#generic_idents),*> }
    } else {
        parse_quote! { #interface_name }
    };

    Ok(ImplFakeMethodScheme {
        common: ImplCommonMethodScheme {
            vis: visibility,
            method_name,
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
    })
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

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

        let scheme = ImplFakeScheme::try_from(info)
            .expect("conversion should succeed for a non-generic impl block");

        assert_eq!(scheme.methods.len(), 1);
        let method = &scheme.methods[0];
        assert_eq!(
            method.common.module_name.to_string(),
            "user_service__get_user_fake_module"
        );
        assert_eq!(method.common.display_name, "get_user");
        assert_eq!(method.common.accessor_name.to_string(), "get_user_fake");
        assert!(method.common.method_generic_params.is_empty());
        assert_eq!(
            method.store_name.to_string(),
            "USER_SERVICE_GET_USER_FAKE_STORE"
        );
        assert_eq!(
            method.interface_name.to_string(),
            "UserServiceGetUserFakeInterface"
        );
        assert_eq!(
            method.interface_type.to_token_stream().to_string(),
            quote::quote!(UserServiceGetUserFakeInterface).to_string()
        );
        // The receiver (`self`) plus the one declared parameter.
        assert_eq!(method.fake_call_values.len(), 2);
        assert!(method.generic_scheme.is_none());
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

        let scheme = ImplFakeScheme::try_from(info).expect("conversion should succeed");

        assert_eq!(scheme.methods.len(), 2);
        assert_eq!(scheme.methods[0].common.display_name, "get_user");
        assert_eq!(scheme.methods[1].common.display_name, "save_user");
        assert_ne!(
            scheme.methods[0].common.module_name,
            scheme.methods[1].common.module_name
        );
        assert_ne!(scheme.methods[0].store_name, scheme.methods[1].store_name);
    }

    #[test]
    fn test_struct_only_generics_are_combined_into_method_scheme() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl<S: 'static> Foo<S> {
                fn bar(&self, x: S) {}
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid generic impl block");

        let scheme = ImplFakeScheme::try_from(info).expect("conversion should succeed");

        let method = &scheme.methods[0];
        let generic_scheme = method
            .generic_scheme
            .as_ref()
            .expect("expected generic_scheme to be Some");
        assert_eq!(generic_scheme.params.len(), 1);
        assert_eq!(
            generic_scheme
                .idents
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>(),
            vec!["S".to_string()]
        );
        // The struct's generics are already in scope from the enclosing `impl<..>` block, so the
        // method's own (empty) generic params are what's redeclared on the accessor function.
        assert!(method.common.method_generic_params.is_empty());
        assert_eq!(
            method.interface_type.to_token_stream().to_string(),
            quote::quote!(FooBarFakeInterface<S>).to_string()
        );
    }

    #[test]
    fn test_method_only_generics_are_combined_into_method_scheme() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl Foo {
                fn bar<M: 'static>(&self, x: M) {}
            }
        };
        let info =
            ImplBlockInfo::try_from(item_impl).expect("valid impl block with generic method");

        let scheme = ImplFakeScheme::try_from(info).expect("conversion should succeed");

        let method = &scheme.methods[0];
        let generic_scheme = method
            .generic_scheme
            .as_ref()
            .expect("expected generic_scheme to be Some");
        assert_eq!(generic_scheme.params.len(), 1);
        assert_eq!(method.common.method_generic_params.len(), 1);
        assert_eq!(
            generic_scheme
                .idents
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>(),
            vec!["M".to_string()]
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

        let scheme = ImplFakeScheme::try_from(info).expect("conversion should succeed");

        let method = &scheme.methods[0];
        let generic_scheme = method
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
        assert_eq!(method.common.method_generic_params.len(), 1);
        assert_eq!(
            method.interface_type.to_token_stream().to_string(),
            quote::quote!(FooBarFakeInterface<S, M>).to_string()
        );
    }

    #[test]
    fn test_const_generic_only_method_excludes_it_from_marker_idents() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl Foo {
                fn bar<const N: usize>(&self, x: [u8; N]) -> usize { N }
            }
        };
        let info = ImplBlockInfo::try_from(item_impl)
            .expect("valid impl block with a const generic method");

        let scheme = ImplFakeScheme::try_from(info).expect("conversion should succeed");

        let method = &scheme.methods[0];
        let generic_scheme = method
            .generic_scheme
            .as_ref()
            .expect("expected Some for a generic method, even with only const generics");
        assert!(generic_scheme.idents_without_const_generics.is_empty());
    }

    #[test]
    fn test_unsupported_param_type_is_rejected() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl Foo {
                fn bar(&self, x: impl Clone) {}
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let result = ImplFakeScheme::try_from(info);

        assert!(
            result.is_err(),
            "expected `impl Trait` param to be rejected"
        );
    }

    #[test]
    fn test_unsupported_call_value_pattern_is_rejected() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl Foo {
                fn bar(&self, _: u32) {}
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let result = ImplFakeScheme::try_from(info);

        assert!(
            result.is_err(),
            "expected a wildcard param pattern to be rejected"
        );
    }
}
