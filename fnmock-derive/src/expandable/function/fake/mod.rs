use crate::{
    expandable::function::{
        FunctionExpandable,
        fake::{
            inline_call::build_inline_call,
            module::{
                fake_store::build_fake_store, implementation_getter::build_implementation_getter,
                interface_getter::build_interface_getter, interface_impl::build_interface_impl,
                interface_struct::build_interface_struct,
            },
        },
    },
    scheme::function::{common::FunctionCommonScheme, fake::FunctionFakeScheme},
};

mod inline_call;
mod module {
    pub mod fake_store;
    pub mod implementation_getter;
    pub mod interface_getter;
    pub mod interface_impl;
    pub mod interface_struct;
}

impl TryFrom<FunctionFakeScheme> for FunctionExpandable {
    type Error = syn::Error;

    fn try_from(value: FunctionFakeScheme) -> Result<Self, Self::Error> {
        let FunctionFakeScheme {
            common,
            store_name,
            interface_name,
            interface_type,
            fn_closure_trait,
            fake_call_values,
            generic_scheme,
        } = value;

        let FunctionCommonScheme {
            vis,
            original,
            module_name,
            display_name,
            accessor_name,
        } = common;

        let accessor_generic_params = generic_scheme
            .as_ref()
            .map(|g| g.params.clone())
            .unwrap_or_default();

        Ok(FunctionExpandable {
            vis,
            original,
            inline_call: build_inline_call(
                &module_name,
                &fake_call_values,
                generic_scheme.as_ref().map(|g| g.idents.as_slice()),
            ),
            accessor_name,
            accessor_generic_params,
            module_name,
            module_parts: vec![
                build_fake_store(
                    &store_name,
                    &display_name,
                    &fn_closure_trait,
                    generic_scheme.as_ref().map(|g| g.params.len()),
                ),
                build_implementation_getter(
                    &store_name,
                    &fn_closure_trait,
                    generic_scheme.as_ref(),
                ),
                build_interface_struct(&interface_name, generic_scheme.as_ref()),
                build_interface_impl(
                    &interface_name,
                    &store_name,
                    generic_scheme.as_ref(),
                    &fn_closure_trait,
                ),
                build_interface_getter(&interface_name, generic_scheme.as_ref()),
            ],
            interface_type,
        })
    }
}
