use crate::{
    expandable::common::{
        fake::module::module_parts::build_module_parts as build_fake_module_parts,
        mock::clear::build_mock_clear,
        spy::module::module_parts::build_module_parts as build_spy_module_parts,
    },
    scheme::{common::generic_scheme::GenericScheme, fake::FakeScheme, spy::SpyScheme},
};

/// Builds both halves' module parts, with their own `clear` suppressed, followed by the mock's
/// combined `clear`. The interface struct and getter are emitted by the caller once per module.
pub fn build_module_parts(
    display_name: &str,
    interface_name: &syn::Ident,
    generic_scheme: Option<&GenericScheme>,
    fake: &FakeScheme,
    spy: &SpyScheme,
) -> Vec<proc_macro2::TokenStream> {
    [
        build_fake_module_parts(display_name, interface_name, generic_scheme, fake, false),
        build_spy_module_parts(display_name, interface_name, generic_scheme, spy, false),
        vec![build_mock_clear(
            interface_name,
            &fake.store_name,
            &spy.store_name,
            generic_scheme,
        )],
    ]
    .concat()
}
