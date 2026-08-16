use syn::parse_quote;

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
            generic_count,
            generic_params,
            generic_idents,
            generic_idents_without_const_generics,
            generic_keys,
        } = value;

        let FunctionCommonScheme {
            vis,
            item_fn,
            module_name,
            display_name,
            accessor_name,
            accessor_generic_params,
        } = common;

        Ok(FunctionExpandable {
            vis,
            item_fn,
            inline_call: build_inline_call(
                &module_name,
                &fake_call_values,
                generic_idents.as_deref(),
            ),
            accessor_name,
            accessor_generic_params,
            module_name,
            module_parts: vec![
                build_fake_store(&store_name, &display_name, &fn_closure_trait, generic_count),
                build_implementation_getter(
                    &store_name,
                    &fn_closure_trait,
                    generic_params.as_deref(),
                    generic_keys.as_deref(),
                ),
                build_interface_struct(
                    &interface_name,
                    generic_params.as_deref(),
                    generic_idents_without_const_generics.as_deref(),
                ),
                build_interface_impl(
                    &interface_name,
                    &store_name,
                    generic_params.as_deref(),
                    generic_idents.as_deref(),
                    generic_keys.as_deref(),
                    &fn_closure_trait,
                ),
                build_interface_getter(
                    &interface_name,
                    generic_params.as_deref(),
                    generic_idents.as_deref(),
                ),
            ],
            interface_type,
        })
    }
}
