use quote::quote;

/// Builds a function closure trait (e.g. `Fn(i32, &str) -> bool`) from a list of parameter types and a return type.
///
/// Make sure to replace any `Self` types in the parameter types and return type with the actual type of `Self` before calling this function, as it does not handle `Self` replacement itself.
///
/// # Params
///
/// - `lifetime_params`: The lifetime parameters of the function / struct + method signature
/// - `params`: The parameter types of the function
/// - `output`: The return type of the function
pub fn build_fn_closure_trait(
    lifetimes: &[syn::Lifetime],
    params: &[syn::Type],
    output: &syn::ReturnType
) -> syn::Result<syn::TraitBound> {
    let fn_ptr_tokens: proc_macro2::TokenStream = if lifetimes.is_empty() {
        quote! { Fn(#(#params),*) #output }
    } else {
        quote! { for<#(#lifetimes),*> Fn(#(#params),*) #output }
    };
    syn::parse2(fn_ptr_tokens)
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

        let bound = build_fn_closure_trait(&lifetimes, &params, &output).expect(
            "expected build_fn_closure_trait to succeed"
        );

        let expected = parse_trait_bound("Fn(i32, &str) -> bool");
        assert_eq!(bound.to_token_stream().to_string(), expected.to_token_stream().to_string());
        assert!(
            !bound.to_token_stream().to_string().trim_start().starts_with("for"),
            "expected no `for<...>` prefix when there are no lifetimes"
        );
    }

    #[test]
    fn test_no_lifetimes_no_params_default_return_has_no_arrow() {
        let lifetimes: Vec<syn::Lifetime> = vec![];
        let params: Vec<syn::Type> = vec![];
        let output = syn::ReturnType::Default;

        let bound = build_fn_closure_trait(&lifetimes, &params, &output).expect(
            "expected build_fn_closure_trait to succeed"
        );

        let expected = parse_trait_bound("Fn()");
        assert_eq!(bound.to_token_stream().to_string(), expected.to_token_stream().to_string());
        assert!(
            !bound.to_token_stream().to_string().trim_start().starts_with("for"),
            "expected no `for<...>` prefix when there are no lifetimes"
        );
    }

    #[test]
    fn test_single_lifetime_adds_for_prefix() {
        let lifetimes: Vec<syn::Lifetime> = vec![syn::parse_quote!('a)];
        let params: Vec<syn::Type> = vec![syn::parse_quote!(&'a str)];
        let output: syn::ReturnType = syn::parse_quote!(-> bool);

        let bound = build_fn_closure_trait(&lifetimes, &params, &output).expect(
            "expected build_fn_closure_trait to succeed"
        );

        let expected = parse_trait_bound("for<'a> Fn(&'a str) -> bool");
        assert_eq!(bound.to_token_stream().to_string(), expected.to_token_stream().to_string());
        assert!(
            bound.to_token_stream().to_string().trim_start().starts_with("for"),
            "expected a `for<'a>` prefix when a lifetime is present"
        );
    }

    #[test]
    fn test_two_lifetimes_adds_for_prefix_with_both() {
        let lifetimes: Vec<syn::Lifetime> = vec![syn::parse_quote!('a), syn::parse_quote!('b)];
        let params: Vec<syn::Type> = vec![];
        let output = syn::ReturnType::Default;

        let bound = build_fn_closure_trait(&lifetimes, &params, &output).expect(
            "expected build_fn_closure_trait to succeed"
        );

        let expected = parse_trait_bound("for<'a, 'b> Fn()");
        assert_eq!(bound.to_token_stream().to_string(), expected.to_token_stream().to_string());
        assert!(
            bound.to_token_stream().to_string().trim_start().starts_with("for"),
            "expected a `for<'a, 'b>` prefix when two lifetimes are present"
        );
    }
}
