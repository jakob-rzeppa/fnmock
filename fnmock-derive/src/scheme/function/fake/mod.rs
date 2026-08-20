use syn::parse_quote;

use crate::{
    item_info::{call_value::CallValue, function::FunctionInfo},
    scheme::{
        common::{fn_closure_trait::build_fn_closure_trait, generic_scheme::GenericScheme},
        function::{
            common::FunctionCommonScheme,
            fake::names::{
                build_accessor_name, build_interface_name, build_module_name, build_store_name,
            },
        },
    },
};

mod names;

pub struct FunctionFakeScheme {
    pub common: FunctionCommonScheme,

    pub store_name: syn::Ident,

    pub fn_closure_trait: syn::TraitBound,

    pub interface_name: syn::Ident,
    pub interface_type: syn::Type,
    pub fake_call_values: Vec<CallValue>,

    pub generic_scheme: Option<GenericScheme>,
}

impl TryFrom<FunctionInfo> for FunctionFakeScheme {
    type Error = syn::Error;

    fn try_from(value: FunctionInfo) -> Result<Self, Self::Error> {
        let module_name = build_module_name(&value.name);
        let store_name = build_store_name(&value.name);
        let accessor_name = build_accessor_name(&value.name);
        let interface_name = build_interface_name(&value.name);
        let display_name = value.name.to_string();

        let param_types = value
            .params
            .iter()
            .map(|p| p.ty.clone())
            .collect::<Vec<_>>();
        let fn_closure_trait =
            build_fn_closure_trait(&value.lifetimes, &param_types, &value.return_type)?;

        let fake_call_values = value
            .params
            .iter()
            .map(|p| CallValue::try_from(&p.pat))
            .collect::<Result<Vec<_>, _>>()?;

        let generic_scheme = if value.generic_params.is_empty() {
            None
        } else {
            let idents_without_const_generics = value
                .generic_params
                .iter()
                .filter_map(|g| match &g.param {
                    syn::GenericParam::Type(type_param) => Some(type_param.ident.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>();

            Some(GenericScheme {
                params: value
                    .generic_params
                    .iter()
                    .map(|g| g.param.clone())
                    .collect(),
                idents: value
                    .generic_params
                    .iter()
                    .map(|g| g.ident.clone())
                    .collect(),
                idents_without_const_generics,
                keys: value.generic_params.iter().map(|g| g.key.clone()).collect(),
            })
        };

        let interface_type: syn::Type = if let Some(generic_scheme) = &generic_scheme {
            let generic_idents = &generic_scheme.idents;
            parse_quote! { #interface_name<#(#generic_idents),*> }
        } else {
            parse_quote! { #interface_name }
        };

        Ok(FunctionFakeScheme {
            common: FunctionCommonScheme {
                vis: value.visibility,
                original: value.original,
                module_name,
                display_name,
                accessor_name,
            },
            store_name,
            fn_closure_trait,
            interface_name,
            interface_type,
            fake_call_values,
            generic_scheme,
        })
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    #[test]
    fn test_standalone_non_generic_function() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn get_user(id: u32) -> String {
                todo!()
            }
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let scheme = FunctionFakeScheme::try_from(info)
            .expect("conversion should succeed for a non-generic function");

        assert_eq!(
            scheme.common.module_name.to_string(),
            "get_user_fake_module"
        );
        assert_eq!(scheme.common.display_name, "get_user");
        assert_eq!(scheme.common.accessor_name.to_string(), "get_user_fake");
        assert_eq!(scheme.store_name.to_string(), "GET_USER_FAKE_STORE");
        assert_eq!(scheme.interface_name.to_string(), "GetUserFakeInterface");
        assert_eq!(
            scheme.interface_type.to_token_stream().to_string(),
            quote::quote!(GetUserFakeInterface).to_string()
        );
        assert_eq!(scheme.fake_call_values.len(), 1);
        assert!(scheme.generic_scheme.is_none());
    }

    #[test]
    fn test_generic_function_carries_generic_info_through() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo<T>(x: T) -> T {
                x
            }
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let scheme = FunctionFakeScheme::try_from(info)
            .expect("conversion should succeed for a generic function");

        let generic_scheme = scheme
            .generic_scheme
            .as_ref()
            .expect("expected generic_scheme to be Some");
        assert_eq!(generic_scheme.params.len(), 1);
        let generic_idents = &generic_scheme.idents;
        assert_eq!(
            generic_idents
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>(),
            vec!["T".to_string()]
        );
        assert_eq!(
            scheme.interface_type.to_token_stream().to_string(),
            quote::quote!(FooFakeInterface<T>).to_string()
        );
    }

    #[test]
    fn test_generic_function_with_only_const_generics_excludes_them_from_marker_idents() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo<const N: usize>(x: [u8; N]) -> usize {
                N
            }
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let scheme = FunctionFakeScheme::try_from(info)
            .expect("conversion should succeed for a const-generic function");

        let generic_scheme = scheme
            .generic_scheme
            .as_ref()
            .expect("expected Some for a generic function, even with only const generics");
        assert!(generic_scheme.idents_without_const_generics.is_empty());
    }

    #[test]
    fn test_unsupported_param_type_is_rejected() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo(x: impl Clone) {}
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let result = FunctionFakeScheme::try_from(info);

        assert!(
            result.is_err(),
            "expected `impl Trait` param to be rejected"
        );
    }

    #[test]
    fn test_unsupported_call_value_pattern_is_rejected() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo(_: u32) {}
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let result = FunctionFakeScheme::try_from(info);

        assert!(
            result.is_err(),
            "expected a wildcard param pattern to be rejected"
        );
    }
}
