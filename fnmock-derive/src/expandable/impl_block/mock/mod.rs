use syn::parse_quote;

use crate::{
    expandable::{
        common::{
            fake::inline_call::build_inline_call as build_fake_inline_call,
            interface::{
                interface_getter::build_interface_getter, interface_struct::build_interface_struct,
            },
            mock::{inline_call::merge_blocks, module_parts::build_module_parts},
            spy::inline_call::build_inline_call as build_spy_inline_call,
        },
        impl_block::{ImplExpandable, ImplMethodExpandable},
    },
    scheme::{
        common::impl_block::{ImplCommonMethodScheme, ImplCommonScheme},
        mock::impl_block::{ImplMockMethodScheme, ImplMockScheme},
    },
};

impl TryFrom<ImplMockScheme> for ImplExpandable {
    type Error = syn::Error;

    fn try_from(value: ImplMockScheme) -> Result<Self, Self::Error> {
        let ImplMockScheme {
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

fn create_impl_method_expandable(scheme: ImplMockMethodScheme) -> ImplMethodExpandable {
    let ImplMockMethodScheme {
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
        fake,
        spy,
    } = scheme;

    let interface_type: syn::Type = if let Some(generic_scheme) = &generic_scheme {
        let generic_idents = &generic_scheme.idents;
        parse_quote! { #interface_name<#(#generic_idents),*> }
    } else {
        parse_quote! { #interface_name }
    };

    let generic_idents = generic_scheme.as_ref().map(|g| g.idents.as_slice());
    // Record first, then the fake: the spy half observes every call, whether or not a fake
    // intercepts it.
    let inline_call = merge_blocks(
        build_spy_inline_call(&module_name, &spy.reference_call_values, generic_idents),
        build_fake_inline_call(&module_name, &fake.fake_call_values, generic_idents),
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
            &spy,
        ),
        vec![build_interface_getter(
            &interface_name,
            generic_scheme.as_ref(),
        )],
    ]
    .concat();

    ImplMethodExpandable {
        vis,
        inline_call,
        accessor_name,
        method_generic_params,
        interface_type,
        module_name,
        module_parts,
    }
}
