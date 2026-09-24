use syn::parse_quote;

use crate::{
    expandable::common::spy::module::{
        interface_impl::build_interface_impl, matcher::build_matcher,
        record_call::build_record_call, spy_store::build_spy_store,
    },
    scheme::{common::generic_scheme::GenericScheme, spy::SpyScheme},
};

/// Builds the spy half's own module parts: the store, the matcher, the interface impl, and `internal_record_call`.
pub fn build_module_parts(
    display_name: &str,
    interface_name: &syn::Ident,
    generic_scheme: Option<&GenericScheme>,
    spy: &SpyScheme,
    include_clear: bool,
) -> Vec<proc_macro2::TokenStream> {
    let matcher_type: syn::Type = if let Some(generic_scheme) = generic_scheme {
        let generic_idents = &generic_scheme.idents;
        let matcher_name = &spy.matcher_name;
        parse_quote! { #matcher_name<#(#generic_idents),*> }
    } else {
        let matcher_name = &spy.matcher_name;
        parse_quote! { #matcher_name }
    };

    vec![
        build_spy_store(
            &spy.store_name,
            display_name,
            &matcher_type,
            generic_scheme.map(|g| g.params.len()),
        ),
        build_matcher(
            &spy.matcher_name,
            &spy.params_name,
            &spy.param_idents,
            &spy.param_types,
            &spy.params_tuple_types,
            generic_scheme,
            spy.supports_expect,
        ),
        build_interface_impl(
            interface_name,
            &spy.store_name,
            &spy.matcher_name,
            display_name,
            &spy.param_idents,
            &spy.param_types,
            generic_scheme,
            &spy.generic_display_fragments,
            spy.supports_expect,
            include_clear,
        ),
        build_record_call(
            &spy.store_name,
            &spy.matcher_name,
            &spy.params_name,
            display_name,
            &spy.param_idents,
            &spy.param_types,
            generic_scheme,
            &spy.generic_display_fragments,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    fn spy_scheme() -> SpyScheme {
        SpyScheme {
            store_name: parse_quote!(MY_STORE),
            matcher_name: parse_quote!(MyMatcher),
            params_name: parse_quote!(MyMatcherParams),
            param_idents: vec![parse_quote!(id)],
            param_types: vec![parse_quote!(u32)],
            params_tuple_types: vec![parse_quote!(u32)],
            reference_call_values: vec![parse_quote!(&id)],
            generic_display_fragments: vec![],
            supports_expect: true,
        }
    }

    #[test]
    fn test_non_generic() {
        let interface_name: syn::Ident = parse_quote!(MyInterface);
        let spy = spy_scheme();

        let parts = build_module_parts("my_fn", &interface_name, None, &spy, true);

        assert_eq!(parts.len(), 4);
    }

    #[test]
    fn test_generic() {
        let interface_name: syn::Ident = parse_quote!(MyInterface);
        let spy = spy_scheme();
        let generic_scheme = GenericScheme {
            params: vec![parse_quote!(T)],
            idents: vec![parse_quote!(T)],
            idents_without_const_generics: vec![parse_quote!(T)],
            keys: vec![parse_quote!(::std::any::TypeId::of::<T>())],
        };

        let parts = build_module_parts("my_fn", &interface_name, Some(&generic_scheme), &spy, true);

        assert_eq!(parts.len(), 4);
    }
}
