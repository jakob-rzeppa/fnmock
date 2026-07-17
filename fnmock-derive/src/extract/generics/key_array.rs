use quote::quote;

use crate::extract::generics::sanitized_params::SanitizedGenericParams;

/// Build a `GenericKeyPart` array: [GenericKeyPart::Type(TypeId::of::<T>()), GenericKeyPart::Const(ConstValue::new(C)), ...]
///
/// Type parameters are keyed by their `TypeId`. Const parameters are keyed by their actual value (via
/// `fnmock::generic_fake_store::ConstValue::new`), not just the `TypeId` of their type — otherwise every
/// value of e.g. `const C: usize` would collapse onto the single key `TypeId::of::<usize>()`.
pub fn build_generic_key_array(
    generic_idents: &SanitizedGenericParams,
) -> syn::Result<Vec<syn::Expr>> {
    generic_idents
        .get_generic_params()
        .iter()
        .map(|param| {
            let ts = match param {
                syn::GenericParam::Type(type_param) => {
                    let ident = &type_param.ident;
                    quote! { ::fnmock::generic_fake_store::key::GenericKeyPart::Type(::std::any::TypeId::of::<#ident>()) }
                }
                syn::GenericParam::Const(const_param) => {
                    let const_ident = &const_param.ident;
                    quote! {
                        ::fnmock::generic_fake_store::key::GenericKeyPart::Const(
                            ::fnmock::generic_fake_store::key::ConstValue::new(#const_ident)
                        )
                    }
                }
                other => {
                    return Err(
                        syn::Error::new_spanned(
                            other,
                            "internal error: expected only type and const parameters when building the generic key array, but found a lifetime parameter. This is a bug in fnmock; please report it."
                        )
                    );
                }
            };

            syn::parse2(ts).map_err(|e|
                syn::Error::new(
                    proc_macro2::Span::mixed_site(),
                    format!(
                        "internal error: failed to build a generic key expression: {e}. This is a bug in fnmock; please report it."
                    )
                )
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    #[test]
    fn test_no_generics_returns_empty_vec() {
        let params = SanitizedGenericParams::new(vec![]).unwrap();

        let keys = build_generic_key_array(&params).expect("expected build to succeed");

        assert!(keys.is_empty());
    }

    #[test]
    fn test_single_type_param_builds_type_key() {
        let keys = build_generic_key_array(
            &SanitizedGenericParams::new(vec![syn::parse_quote!(T)]).unwrap(),
        )
        .expect("expected build to succeed");

        assert_eq!(keys.len(), 1);

        let expected = quote! {
            ::fnmock::generic_fake_store::key::GenericKeyPart::Type(::std::any::TypeId::of::<T>())
        };

        assert_eq!(
            keys[0].to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_single_const_param_builds_const_key() {
        let keys = build_generic_key_array(
            &SanitizedGenericParams::new(vec![syn::parse_quote!(const N: usize)]).unwrap(),
        )
        .expect("expected build to succeed");

        assert_eq!(keys.len(), 1);

        let expected = quote! {
            ::fnmock::generic_fake_store::key::GenericKeyPart::Const(
                ::fnmock::generic_fake_store::key::ConstValue::new(N)
            )
        };

        assert_eq!(
            keys[0].to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_mixed_type_and_const_params_builds_in_order() {
        let keys = build_generic_key_array(
            &SanitizedGenericParams::new(vec![
                syn::parse_quote!(T),
                syn::parse_quote!(const N: usize),
            ])
            .unwrap(),
        )
        .expect("expected build to succeed");

        assert_eq!(keys.len(), 2);

        let expected_type = quote! {
            ::fnmock::generic_fake_store::key::GenericKeyPart::Type(::std::any::TypeId::of::<T>())
        };
        let expected_const = quote! {
            ::fnmock::generic_fake_store::key::GenericKeyPart::Const(
                ::fnmock::generic_fake_store::key::ConstValue::new(N)
            )
        };

        assert_eq!(
            keys[0].to_token_stream().to_string(),
            expected_type.to_token_stream().to_string()
        );
        assert_eq!(
            keys[1].to_token_stream().to_string(),
            expected_const.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_multiple_type_params_preserve_order() {
        let params =
            SanitizedGenericParams::new(vec![syn::parse_quote!(A), syn::parse_quote!(B)]).unwrap();

        let keys = build_generic_key_array(&params).expect("expected build to succeed");

        assert_eq!(keys.len(), 2);
        let expected_a = quote! {
            ::fnmock::generic_fake_store::key::GenericKeyPart::Type(::std::any::TypeId::of::<A>())
        };
        let expected_b = quote! {
            ::fnmock::generic_fake_store::key::GenericKeyPart::Type(::std::any::TypeId::of::<B>())
        };
        assert_eq!(
            keys[0].to_token_stream().to_string(),
            expected_a.to_token_stream().to_string()
        );
        assert_eq!(
            keys[1].to_token_stream().to_string(),
            expected_b.to_token_stream().to_string()
        );
    }
}
