use quote::quote;

use crate::{
    expandable::function::spy::module::matcher::build_params_marker_construct,
    scheme::common::generic_scheme::GenericScheme,
};

/// Builds `internal_record_call`, the module-level function the injected call recording invokes
/// directly (not through the interface) to hand the spy its arguments.
///
/// It takes one argument per parameter, rather than a single params value, and builds the
/// `#params_name` wrapper itself: constructing that wrapper is only legal from inside this
/// module (its fields aren't `pub`; see [`build_matcher`](super::matcher::build_matcher)'s doc
/// comment for why), and this is the one place that needs to.
pub fn build_record_call(
    store_name: &syn::Ident,
    matcher_name: &syn::Ident,
    params_name: &syn::Ident,
    display_name: &str,
    param_idents: &[syn::Ident],
    param_types: &[syn::Type],
    generic_scheme: Option<&GenericScheme>,
    generic_display_fragments: &[syn::Expr],
) -> proc_macro2::TokenStream {
    let call_params = param_idents
        .iter()
        .zip(param_types)
        .map(|(ident, ty)| quote! { #ident: &#ty, });
    let params_marker_construct =
        build_params_marker_construct(generic_scheme, !param_idents.is_empty());
    let params_construct = quote! {
        #params_name(#(#param_idents,)* #params_marker_construct)
    };

    if let Some(generic_scheme) = generic_scheme {
        let generic_params = &generic_scheme.params;
        let generic_idents = &generic_scheme.idents;
        let generic_keys = &generic_scheme.keys;
        let matcher_type = quote! { #matcher_name<#(#generic_idents),*> };

        quote! {
            pub(super) fn internal_record_call<#(#generic_params),*>(#(#call_params)*) {
                let params = #params_construct;
                #store_name.with_borrow_mut(|store| {
                    store.with_store_mut::<#matcher_type, _>(
                        [#(#generic_keys),*],
                        || format!("{}::<{}>", #display_name, [#(#generic_display_fragments),*].join(", ")),
                        |spy| spy.record_call(&params),
                    )
                })
            }
        }
    } else {
        quote! {
            pub(super) fn internal_record_call(#(#call_params)*) {
                let params = #params_construct;
                #store_name.with_borrow_mut(|spy| {
                    spy.record_call(&params);
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_non_generic_multiple_params() {
        let store_name: syn::Ident = parse_quote!(GET_USER_SPY_STORE);
        let matcher_name: syn::Ident = parse_quote!(GetUserMatcher);
        let params_name: syn::Ident = parse_quote!(GetUserMatcherParams);
        let param_idents: Vec<syn::Ident> = vec![parse_quote!(id), parse_quote!(uuid)];
        let param_types: Vec<syn::Type> = vec![parse_quote!(String), parse_quote!(str)];

        let res = build_record_call(
            &store_name,
            &matcher_name,
            &params_name,
            "get_user",
            &param_idents,
            &param_types,
            None,
            &[],
        );

        let expected = quote! {
            pub(super) fn internal_record_call(id: &String, uuid: &str,) {
                let params = GetUserMatcherParams(id, uuid,);
                GET_USER_SPY_STORE.with_borrow_mut(|spy| {
                    spy.record_call(&params);
                })
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_non_generic_zero_params() {
        let store_name: syn::Ident = parse_quote!(PING_SPY_STORE);
        let matcher_name: syn::Ident = parse_quote!(PingMatcher);
        let params_name: syn::Ident = parse_quote!(PingMatcherParams);

        let res = build_record_call(
            &store_name,
            &matcher_name,
            &params_name,
            "ping",
            &[],
            &[],
            None,
            &[],
        );

        let expected = quote! {
            pub(super) fn internal_record_call() {
                let params = PingMatcherParams(::std::marker::PhantomData,);
                PING_SPY_STORE.with_borrow_mut(|spy| {
                    spy.record_call(&params);
                })
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_single_param() {
        let store_name: syn::Ident = parse_quote!(FOO_SPY_STORE);
        let matcher_name: syn::Ident = parse_quote!(FooMatcher);
        let params_name: syn::Ident = parse_quote!(FooMatcherParams);
        let param_idents: Vec<syn::Ident> = vec![parse_quote!(a)];
        let param_types: Vec<syn::Type> = vec![parse_quote!(T)];
        let generic_scheme = GenericScheme {
            params: vec![parse_quote!(T: 'static)],
            idents: vec![parse_quote!(T)],
            idents_without_const_generics: vec![parse_quote!(T)],
            keys: vec![parse_quote! {
                ::fnmock::generic_fake_store::key::GenericKeyPart::Type(::std::any::TypeId::of::<T>())
            }],
        };
        let display_fragments: Vec<syn::Expr> =
            vec![parse_quote! { ::std::any::type_name::<T>().to_string() }];

        let res = build_record_call(
            &store_name,
            &matcher_name,
            &params_name,
            "foo",
            &param_idents,
            &param_types,
            Some(&generic_scheme),
            &display_fragments,
        );

        let expected = quote! {
            pub(super) fn internal_record_call<T: 'static>(a: &T,) {
                let params = FooMatcherParams(a, ::std::marker::PhantomData,);
                FOO_SPY_STORE.with_borrow_mut(|store| {
                    store.with_store_mut::<FooMatcher<T>, _>(
                        [::fnmock::generic_fake_store::key::GenericKeyPart::Type(::std::any::TypeId::of::<T>())],
                        || format!("{}::<{}>", "foo", [::std::any::type_name::<T>().to_string()].join(", ")),
                        |spy| spy.record_call(&params),
                    )
                })
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }
}
