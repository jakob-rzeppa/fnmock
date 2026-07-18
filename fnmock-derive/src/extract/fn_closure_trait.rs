//! Construction of the `Fn(..) -> ..` trait bound that a fake for a given signature must satisfy.

use quote::quote;
use syn::Type;

/// Builds a function closure trait (e.g. `Fn(i32, &str) -> bool`) from a list of parameter types and a return type.
///
/// Make sure to replace any `Self` types in the parameter types and return type with the actual type of `Self` before calling this function, as it does not handle `Self` replacement itself.
///
/// # Params
///
/// - `lifetime_params`: The lifetime parameters of the function / struct + method signature
/// - `params`: The parameter types of the function
/// The lifetimes are bound higher-ranked (`for<'a> Fn(&'a str)`), because the fake is stored as a
/// single `dyn Fn` value and so cannot be generic over lifetimes the way the faked function is.
///
/// This is also where a signature is checked for types fnmock cannot fake, so that the user gets a
/// spanned error on their own code rather than a confusing one on generated code.
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
    // Check if the types are supported.
    for param in params {
        check_if_type_is_supported_for_fn_closure(param)?;
    }
    if let syn::ReturnType::Type(_, ty) = output {
        check_if_type_is_supported_for_fn_closure(ty)?;
    }

    let fn_ptr_tokens: proc_macro2::TokenStream = if lifetimes.is_empty() {
        quote! { Fn(#(#params),*) #output }
    } else {
        quote! { for<#(#lifetimes),*> Fn(#(#params),*) #output }
    };
    syn::parse2(fn_ptr_tokens)
}

/// Reject types that cannot appear in a fake's closure trait.
///
/// The match is written out over every `syn::Type` variant rather than falling back to a catch-all
/// `Ok(())`, so that a variant syn adds later surfaces as a report-this-bug error instead of
/// silently producing generated code that does not compile.
fn check_if_type_is_supported_for_fn_closure(ty: &Type) -> Result<(), syn::Error> {
    match ty {
        // A fixed size array type: `[T; n]`.
        syn::Type::Array(_) => Ok(()),

        // A bare function type: `fn(usize) -> bool`.
        syn::Type::BareFn(_) => Ok(()),

        // A type contained within invisible delimiters.
        syn::Type::Group(_) => Ok(()),

        // An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
        // a lifetime.
        syn::Type::ImplTrait(_) => Err(syn::Error::new_spanned(
            ty,
            "The #[fakeable] attribute does not support `impl Trait` in a function signature. Please use a concrete type or a generic type parameter instead.",
        )),

        // Indication that a type should be inferred by the compiler: `_`.
        syn::Type::Infer(_) => Err(syn::Error::new_spanned(
            ty,
            "The #[fakeable] attribute does not support the inferred type `_` in a function signature. Please specify the type explicitly.",
        )),

        // A macro in the type position.
        syn::Type::Macro(_) => Err(syn::Error::new_spanned(
            ty,
            "The #[fakeable] attribute does not support macros in a function signature. Please use a concrete type or a generic type parameter instead.",
        )),

        // The never type: `!`.
        syn::Type::Never(_) => Err(syn::Error::new_spanned(
            ty,
            "The #[fakeable] attribute does not support the never type `!` in a function signature.",
        )),

        // A parenthesized type equivalent to the inner type.
        syn::Type::Paren(_) => Ok(()),

        // A path type: `core::slice::Iter`. Can be optionally qualified with a
        // self-type as in `<Vec<T> as SomeTrait>::Associated`.
        syn::Type::Path(_) => Ok(()),

        // A raw pointer type: `*const T` or `*mut T`.
        syn::Type::Ptr(_) => Ok(()),

        // A reference type: `&'a T` or `&'a mut T`.
        syn::Type::Reference(_) => Ok(()),

        // A dynamically sized slice type: `[T]`.
        syn::Type::Slice(_) => Ok(()),

        // A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
        // trait or a lifetime.
        syn::Type::TraitObject(_) => Ok(()),

        // A tuple type: `(T1, T2, T3)`.
        syn::Type::Tuple(_) => Ok(()),

        _ => Err(syn::Error::new_spanned(
            ty,
            "The #[fakeable] attribute does not support this type in a function signature. This is probably a type added to syn after the last check for supported types was implemented. Please report this as a bug to the fnmock project.",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    /// Parses `input` as a `syn::TraitBound` to build the expected value for a test.
    ///
    /// Comparing two parsed-and-re-emitted `TraitBound`s (rather than one parsed value against a
    /// literal `quote!` token stream) keeps token-spacing normalization identical on both sides of
    /// the assertion, since re-emitting a parsed AST can space tokens differently than a literal
    /// `quote!` invocation (e.g. `for<'a>` re-emits as `for < 'a >`).
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
        assert!(
            !bound
                .to_token_stream()
                .to_string()
                .trim_start()
                .starts_with("for"),
            "expected no `for<...>` prefix when there are no lifetimes"
        );
    }

    #[test]
    fn test_no_lifetimes_no_params_default_return_has_no_arrow() {
        let lifetimes: Vec<syn::Lifetime> = vec![];
        let params: Vec<syn::Type> = vec![];
        let output = syn::ReturnType::Default;

        let bound = build_fn_closure_trait(&lifetimes, &params, &output)
            .expect("expected build_fn_closure_trait to succeed");

        let expected = parse_trait_bound("Fn()");
        assert_eq!(
            bound.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
        assert!(
            !bound
                .to_token_stream()
                .to_string()
                .trim_start()
                .starts_with("for"),
            "expected no `for<...>` prefix when there are no lifetimes"
        );
    }

    #[test]
    fn test_single_lifetime_adds_for_prefix() {
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
        assert!(
            bound
                .to_token_stream()
                .to_string()
                .trim_start()
                .starts_with("for"),
            "expected a `for<'a>` prefix when a lifetime is present"
        );
    }

    #[test]
    fn test_two_lifetimes_adds_for_prefix_with_both() {
        let lifetimes: Vec<syn::Lifetime> = vec![syn::parse_quote!('a), syn::parse_quote!('b)];
        let params: Vec<syn::Type> = vec![];
        let output = syn::ReturnType::Default;

        let bound = build_fn_closure_trait(&lifetimes, &params, &output)
            .expect("expected build_fn_closure_trait to succeed");

        let expected = parse_trait_bound("for<'a, 'b> Fn()");
        assert_eq!(
            bound.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
        assert!(
            bound
                .to_token_stream()
                .to_string()
                .trim_start()
                .starts_with("for"),
            "expected a `for<'a, 'b>` prefix when two lifetimes are present"
        );
    }

    // `check_if_type_is_supported_for_fn_closure` is private, so it is exercised indirectly
    // through `build_fn_closure_trait`. Only a couple of unsupported-type variants are checked
    // here (rather than every variant in the `match`) since the goal is to verify that
    // `build_fn_closure_trait` propagates the rejection, not to re-verify the full type match.

    #[test]
    fn test_unsupported_param_type_is_rejected() {
        let lifetimes: Vec<syn::Lifetime> = vec![];
        let params: Vec<syn::Type> = vec![syn::parse_quote!(impl Clone)];
        let output = syn::ReturnType::Default;

        let result = build_fn_closure_trait(&lifetimes, &params, &output);

        assert!(
            result.is_err(),
            "expected an `impl Trait` parameter type to be rejected"
        );
    }

    #[test]
    fn test_unsupported_return_type_is_rejected() {
        let lifetimes: Vec<syn::Lifetime> = vec![];
        let params: Vec<syn::Type> = vec![];
        let output: syn::ReturnType = syn::parse_quote!(-> !);

        let result = build_fn_closure_trait(&lifetimes, &params, &output);

        assert!(
            result.is_err(),
            "expected the never type `!` as a return type to be rejected"
        );
    }

    #[test]
    fn test_supported_param_type_after_unsupported_short_circuits() {
        // Only the first unsupported param needs to be reported; `build_fn_closure_trait`
        // should not attempt to check later params (or the output) once one has failed.
        let lifetimes: Vec<syn::Lifetime> = vec![];
        let params: Vec<syn::Type> = vec![syn::parse_quote!(_), syn::parse_quote!(i32)];
        let output = syn::ReturnType::Default;

        let result = build_fn_closure_trait(&lifetimes, &params, &output);

        assert!(
            result.is_err(),
            "expected the inferred type `_` in an earlier param to be rejected"
        );
    }
}
