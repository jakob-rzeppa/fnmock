use syn::parse_quote;

use crate::{
    expandable::{
        common::{
            interface::{
                interface_getter::build_interface_getter, interface_struct::build_interface_struct,
            },
            spy::{inline_call::build_inline_call, module::module_parts::build_module_parts},
        },
        function::FunctionExpandable,
    },
    scheme::{common::function::FunctionCommonScheme, spy::function::FunctionSpyScheme},
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
            spy,
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

        let inline_call = build_inline_call(
            &module_name,
            &spy.reference_call_values,
            generic_scheme.as_ref().map(|g| g.idents.as_slice()),
        );

        let module_parts = [
            vec![build_interface_struct(
                &interface_name,
                generic_scheme.as_ref(),
            )],
            build_module_parts(
                &display_name,
                &interface_name,
                generic_scheme.as_ref(),
                &spy,
            ),
            vec![build_interface_getter(
                &interface_name,
                generic_scheme.as_ref(),
            )],
        ]
        .concat();

        Ok(FunctionExpandable {
            vis,
            original,
            inline_call,
            accessor_name,
            accessor_generic_params,
            interface_type,
            module_name,
            module_parts,
        })
    }
}
