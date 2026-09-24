use syn::parse_quote;

use crate::{
    expandable::{
        common::{
            fake::{inline_call::build_inline_call, module::module_parts::build_module_parts},
            interface::{
                interface_getter::build_interface_getter, interface_struct::build_interface_struct,
            },
        },
        function::FunctionExpandable,
    },
    scheme::{common::function::FunctionCommonScheme, fake::function::FunctionFakeScheme},
};

impl TryFrom<FunctionFakeScheme> for FunctionExpandable {
    type Error = syn::Error;

    fn try_from(value: FunctionFakeScheme) -> Result<Self, Self::Error> {
        let FunctionFakeScheme {
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
            fake,
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
            &fake.fake_call_values,
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
                &fake,
                true,
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
            module_name,
            module_parts,
            interface_type,
        })
    }
}
