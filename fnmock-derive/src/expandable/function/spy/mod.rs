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
        function::FunctionExpandable,
    },
    scheme::function::{common::FunctionCommonScheme, spy::FunctionSpyScheme},
};

impl TryFrom<FunctionSpyScheme> for FunctionExpandable {
    type Error = syn::Error;

    fn try_from(value: FunctionSpyScheme) -> Result<Self, Self::Error> {
        let FunctionSpyScheme {
            common:
                FunctionCommonScheme {
                    vis,
                    original,
                    module_name,
                    display_name,
                    accessor_name,
                    interface_name,
                    generic_scheme,
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
        } = value;

        let accessor_generic_params = generic_scheme
            .as_ref()
            .map(|g| g.params.clone())
            .unwrap_or_default();
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

        let module_parts = vec![
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
        ];

        Ok(FunctionExpandable {
            vis,
            original,
            inline_call: build_inline_call(
                &module_name,
                &reference_call_values,
                generic_scheme.as_ref().map(|g| g.idents.as_slice()),
            ),
            accessor_name,
            accessor_generic_params,
            interface_type,
            module_name,
            module_parts,
        })
    }
}
