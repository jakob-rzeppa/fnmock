use crate::{
    expandable::common::fake::module::{
        fake_store::build_fake_store, implementation_getter::build_implementation_getter,
        interface_impl::build_interface_impl,
    },
    scheme::{common::generic_scheme::GenericScheme, fake::FakeScheme},
};

/// Builds the fake half's own module parts: the store, the implementation getter, and the
/// interface impl. The interface struct and getter are shared with the spy half and are emitted
/// by the caller once per module.
pub fn build_module_parts(
    display_name: &str,
    interface_name: &syn::Ident,
    generic_scheme: Option<&GenericScheme>,
    fake: &FakeScheme,
    include_clear: bool,
) -> Vec<proc_macro2::TokenStream> {
    vec![
        build_fake_store(
            &fake.store_name,
            display_name,
            &fake.fn_closure_trait,
            generic_scheme.map(|g| g.params.len()),
        ),
        build_implementation_getter(&fake.store_name, &fake.fn_closure_trait, generic_scheme),
        build_interface_impl(
            interface_name,
            &fake.store_name,
            generic_scheme,
            &fake.fn_closure_trait,
            include_clear,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;
    use crate::item_info::call_value::CallValue;

    fn fake_scheme() -> FakeScheme {
        FakeScheme {
            store_name: parse_quote!(MY_STORE),
            fn_closure_trait: parse_quote!(Fn(u32) -> String),
            fake_call_values: vec![CallValue::Ident(parse_quote!(id))],
        }
    }

    #[test]
    fn test_non_generic() {
        let interface_name: syn::Ident = parse_quote!(MyInterface);
        let fake = fake_scheme();

        let parts = build_module_parts("my_fn", &interface_name, None, &fake, true);

        assert_eq!(parts.len(), 3);
    }

    #[test]
    fn test_generic() {
        let interface_name: syn::Ident = parse_quote!(MyInterface);
        let fake = fake_scheme();
        let generic_scheme = GenericScheme {
            params: vec![parse_quote!(T)],
            idents: vec![parse_quote!(T)],
            idents_without_const_generics: vec![parse_quote!(T)],
            keys: vec![parse_quote!(::std::any::TypeId::of::<T>())],
        };

        let parts =
            build_module_parts("my_fn", &interface_name, Some(&generic_scheme), &fake, true);

        assert_eq!(parts.len(), 3);
    }
}
