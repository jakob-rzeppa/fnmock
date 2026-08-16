//! Construction of the `Fn(..) -> ..` trait bound that a fake for a given signature must satisfy.

use quote::quote;

/// Builds a function closure trait (e.g. `Fn(i32, &str) -> bool`) from a list of parameter types
/// and a return type.
///
/// The lifetimes are bound higher-ranked (`for<'a> Fn(&'a str)`), because the fake is stored as a
/// single `dyn Fn` value and so cannot be generic over lifetimes the way the faked function is.
///
/// # Errors
///
/// Returns a spanned error if a parameter or the return type uses a type that cannot appear in a
/// fake's closure trait: `impl Trait`, the inferred type `_`, a macro in type position, or the
/// never type `!`.
pub fn build_fn_closure_trait(
    lifetimes: &[syn::Lifetime],
    params: &[syn::Type],
    output: &syn::ReturnType,
) -> syn::Result<syn::TraitBound> {
    for param in params {
        check_type_is_supported(param)?;
    }
    if let syn::ReturnType::Type(_, ty) = output {
        check_type_is_supported(ty)?;
    }

    let fn_trait_tokens: proc_macro2::TokenStream = if lifetimes.is_empty() {
        quote! { Fn(#(#params),*) #output }
    } else {
        quote! { for<#(#lifetimes),*> Fn(#(#params),*) #output }
    };
    syn::parse2(fn_trait_tokens)
}

/// Reject types that cannot appear in a fake's closure trait.
fn check_type_is_supported(ty: &syn::Type) -> syn::Result<()> {
    match ty {
        syn::Type::ImplTrait(_) => Err(syn::Error::new_spanned(
            ty,
            "#[fakeable] does not support `impl Trait` in a function signature. Please use a concrete type or a generic type parameter instead.",
        )),
        syn::Type::Infer(_) => Err(syn::Error::new_spanned(
            ty,
            "#[fakeable] does not support the inferred type `_` in a function signature. Please specify the type explicitly.",
        )),
        syn::Type::Macro(_) => Err(syn::Error::new_spanned(
            ty,
            "#[fakeable] does not support macros in a function signature. Please use a concrete type or a generic type parameter instead.",
        )),
        syn::Type::Never(_) => Err(syn::Error::new_spanned(
            ty,
            "#[fakeable] does not support the never type `!` in a function signature.",
        )),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    fn parse_trait_bound(input: &str) -> syn::TraitBound {
        syn::parse_str(input).expect("expected input to parse as a syn::TraitBound")
    }

    #[test]
    fn test_no_lifetimes_builds_fn_trait_with_params_and_return() {
        let lifetimes: Vec<syn::Lifetime> = vec![];
        let params: Vec<syn::Type> = vec![syn::parse_quote!(i32), syn::parse_quote!(&str)];
        let output: syn::ReturnType = syn::parse_quote!(-> bool);

        let bound = build_fn_closure_trait(&lifetimes, &params, &output)
            .expect("expected build_fn_closure_trait to succeed");

        let expected = parse_trait_bound("Fn(i32, &str) -> bool");
        assert_eq!(
            bound.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_lifetime_adds_for_prefix() {
        let lifetimes: Vec<syn::Lifetime> = vec![syn::parse_quote!('a)];
        let params: Vec<syn::Type> = vec![syn::parse_quote!(&'a str)];
        let output: syn::ReturnType = syn::parse_quote!(-> bool);

        let bound = build_fn_closure_trait(&lifetimes, &params, &output)
            .expect("expected build_fn_closure_trait to succeed");

        let expected = parse_trait_bound("for<'a> Fn(&'a str) -> bool");
        assert_eq!(
            bound.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_impl_trait_param_is_rejected() {
        let lifetimes: Vec<syn::Lifetime> = vec![];
        let params: Vec<syn::Type> = vec![syn::parse_quote!(impl Clone)];
        let output = syn::ReturnType::Default;

        let result = build_fn_closure_trait(&lifetimes, &params, &output);

        assert!(result.is_err(), "expected `impl Trait` param to be rejected");
    }

    #[test]
    fn test_never_type_return_is_rejected() {
        let lifetimes: Vec<syn::Lifetime> = vec![];
        let params: Vec<syn::Type> = vec![];
        let output: syn::ReturnType = syn::parse_quote!(-> !);

        let result = build_fn_closure_trait(&lifetimes, &params, &output);

        assert!(result.is_err(), "expected `!` return type to be rejected");
    }
}
