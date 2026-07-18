use quote::{quote, ToTokens};

#[derive(Clone)]
pub enum FakeCallValue {
    Ident(syn::Ident),
    Tuple(Vec<FakeCallValue>),
    Slice(Vec<FakeCallValue>),
}

impl ToTokens for FakeCallValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FakeCallValue::Ident(ident) => {
                ident.to_tokens(tokens);
            }
            FakeCallValue::Tuple(elements) => {
                let element_tokens = elements.iter().map(|e| e.to_token_stream());
                let tuple_tokens = quote! { (#(#element_tokens),*) };
                tuple_tokens.to_tokens(tokens);
            }
            FakeCallValue::Slice(elements) => {
                let element_tokens = elements.iter().map(|e| e.to_token_stream());
                let slice_tokens = quote! { [#(#element_tokens),*] };
                slice_tokens.to_tokens(tokens);
            }
        }
    }
}

impl TryFrom<&syn::Pat> for FakeCallValue {
    type Error = syn::Error;

    fn try_from(pat: &syn::Pat) -> Result<Self, Self::Error> {
        match pat {
            syn::Pat::Ident(pat_ident) => {
                // If the pattern uses `ref ident`, we cannot use it in the fakes, since the signature of the fake function will need a value, not a reference and we cannot obtain a value from a reference in the general case.
                if pat_ident.by_ref.is_some() {
                    return Err(
                        syn::Error::new_spanned(
                            pat_ident,
                            "The `ref` keyword is not supported for fake call values. Please use the identifier directly without `ref` (e.g. `ident` instead of `ref ident`)."
                        )
                    );
                }

                // We need to ignore the mutability in the pattern and just get the identifier name for the fake call value.
                Ok(FakeCallValue::Ident(pat_ident.ident.clone()))
            }
            syn::Pat::Tuple(pat_tuple) => {
                let elements = pat_tuple
                    .elems
                    .iter()
                    .map(FakeCallValue::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(FakeCallValue::Tuple(elements))
            }
            syn::Pat::Slice(slice) => {
                let elements = slice
                    .elems
                    .iter()
                    .map(FakeCallValue::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(FakeCallValue::Slice(elements))
            }
            syn::Pat::Struct(pat_struct) => Err(syn::Error::new_spanned(
                pat_struct,
                "Struct destructuring patterns are not supported for fake call values",
            )),
            syn::Pat::TupleStruct(pat_tuple_struct) => Err(syn::Error::new_spanned(
                pat_tuple_struct,
                "Tuple struct destructuring patterns are not supported for fake call values",
            )),
            syn::Pat::Macro(pat_macro) => Err(syn::Error::new_spanned(
                pat_macro,
                "Macro patterns are not supported for fake call values",
            )),
            syn::Pat::Wild(pat_wild) => Err(syn::Error::new_spanned(
                pat_wild,
                "Wildcard patterns are not supported for fake call values",
            )),
            _ => Err(syn::Error::new_spanned(
                pat,
                "Unsupported pattern type for fake call values",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse::Parser;

    fn parse_pat(tokens: proc_macro2::TokenStream) -> syn::Pat {
        syn::Pat::parse_single
            .parse2(tokens)
            .expect("test input should be a valid syn::Pat")
    }

    #[test]
    fn test_plain_ident_pattern_becomes_ident_value() {
        let pat = parse_pat(quote!(x));

        let value =
            FakeCallValue::try_from(&pat).expect("a plain identifier pattern should be accepted");

        match value {
            FakeCallValue::Ident(ident) => assert_eq!(ident, "x"),
            FakeCallValue::Tuple(_) => panic!("expected FakeCallValue::Ident, got Tuple"),
            FakeCallValue::Slice(_) => panic!("expected FakeCallValue::Ident, got Slice"),
        }
    }

    #[test]
    fn test_mut_ident_pattern_ignores_mutability() {
        let pat = parse_pat(quote!(mut x));

        let value = FakeCallValue::try_from(&pat)
            .expect("a `mut` identifier pattern should be accepted, ignoring mutability");

        match value {
            FakeCallValue::Ident(ident) => assert_eq!(ident, "x"),
            FakeCallValue::Tuple(_) => panic!("expected FakeCallValue::Ident, got Tuple"),
            FakeCallValue::Slice(_) => panic!("expected FakeCallValue::Ident, got Slice"),
        }
    }

    #[test]
    fn test_single_level_tuple_pattern_becomes_tuple_of_idents() {
        let pat = parse_pat(quote!((a, b)));

        let value = FakeCallValue::try_from(&pat)
            .expect("a single-level tuple pattern of identifiers should be accepted");

        match value {
            FakeCallValue::Tuple(elements) => {
                assert_eq!(elements.len(), 2);
                match &elements[0] {
                    FakeCallValue::Ident(ident) => assert_eq!(ident, "a"),
                    FakeCallValue::Tuple(_) => panic!("expected element 0 to be Ident, got Tuple"),
                    FakeCallValue::Slice(_) => panic!("expected element 0 to be Ident, got Slice"),
                }
                match &elements[1] {
                    FakeCallValue::Ident(ident) => assert_eq!(ident, "b"),
                    FakeCallValue::Tuple(_) => panic!("expected element 1 to be Ident, got Tuple"),
                    FakeCallValue::Slice(_) => panic!("expected element 1 to be Ident, got Slice"),
                }
            }
            FakeCallValue::Ident(_) => panic!("expected FakeCallValue::Tuple, got Ident"),
            FakeCallValue::Slice(_) => panic!("expected FakeCallValue::Tuple, got Slice"),
        }
    }

    #[test]
    fn test_nested_tuple_pattern_recurses_into_inner_tuples() {
        let pat = parse_pat(quote!(((a, b), c)));

        let value =
            FakeCallValue::try_from(&pat).expect("a nested tuple pattern should be accepted");

        match value {
            FakeCallValue::Tuple(outer) => {
                assert_eq!(outer.len(), 2);

                match &outer[0] {
                    FakeCallValue::Tuple(inner) => {
                        assert_eq!(inner.len(), 2);
                        match &inner[0] {
                            FakeCallValue::Ident(ident) => assert_eq!(ident, "a"),
                            FakeCallValue::Tuple(_) => {
                                panic!("expected inner element 0 to be Ident, got Tuple")
                            }
                            FakeCallValue::Slice(_) => {
                                panic!("expected inner element 0 to be Ident, got Slice")
                            }
                        }
                        match &inner[1] {
                            FakeCallValue::Ident(ident) => assert_eq!(ident, "b"),
                            FakeCallValue::Tuple(_) => {
                                panic!("expected inner element 1 to be Ident, got Tuple")
                            }
                            FakeCallValue::Slice(_) => {
                                panic!("expected inner element 1 to be Ident, got Slice")
                            }
                        }
                    }
                    FakeCallValue::Ident(_) => {
                        panic!("expected outer element 0 to be Tuple, got Ident")
                    }
                    FakeCallValue::Slice(_) => {
                        panic!("expected outer element 0 to be Tuple, got Slice")
                    }
                }

                match &outer[1] {
                    FakeCallValue::Ident(ident) => assert_eq!(ident, "c"),
                    FakeCallValue::Tuple(_) => {
                        panic!("expected outer element 1 to be Ident, got Tuple")
                    }
                    FakeCallValue::Slice(_) => {
                        panic!("expected outer element 1 to be Ident, got Slice")
                    }
                }
            }
            FakeCallValue::Ident(_) => panic!("expected FakeCallValue::Tuple, got Ident"),
            FakeCallValue::Slice(_) => panic!("expected FakeCallValue::Tuple, got Slice"),
        }
    }

    #[test]
    fn test_ref_ident_pattern_is_rejected_with_message_mentioning_ref() {
        let pat = parse_pat(quote!(ref x));

        let result = FakeCallValue::try_from(&pat);

        let Err(error) = result else {
            panic!("a `ref` identifier pattern should be rejected");
        };
        let message = error.to_string().to_lowercase();
        assert!(
            message.contains("ref"),
            "error message should mention `ref`, got: {message}"
        );
    }

    #[test]
    fn test_struct_pattern_is_rejected() {
        let pat = parse_pat(quote!(Foo { a, b }));

        let result = FakeCallValue::try_from(&pat);

        assert!(
            result.is_err(),
            "a struct destructuring pattern should be rejected"
        );
    }

    #[test]
    fn test_tuple_struct_pattern_is_rejected() {
        let pat = parse_pat(quote!(Foo(a, b)));

        let result = FakeCallValue::try_from(&pat);

        assert!(
            result.is_err(),
            "a tuple-struct destructuring pattern should be rejected"
        );
    }

    #[test]
    fn test_macro_pattern_is_rejected() {
        let pat = parse_pat(quote!(m!()));

        let result = FakeCallValue::try_from(&pat);

        assert!(result.is_err(), "a macro pattern should be rejected");
    }

    #[test]
    fn test_wildcard_pattern_is_rejected() {
        let pat = parse_pat(quote!(_));

        let result = FakeCallValue::try_from(&pat);

        assert!(result.is_err(), "a wildcard pattern should be rejected");
    }

    #[test]
    fn test_ident_value_renders_as_bare_ident() {
        let pat = parse_pat(quote!(x));
        let value = FakeCallValue::try_from(&pat).expect("plain identifier pattern should parse");

        assert_eq!(value.to_token_stream().to_string(), quote!(x).to_string());
    }

    #[test]
    fn test_tuple_value_renders_like_a_tuple_expression() {
        let pat = parse_pat(quote!((a, b)));
        let value = FakeCallValue::try_from(&pat).expect("tuple pattern should parse");

        assert_eq!(
            value.to_token_stream().to_string(),
            quote!((a, b)).to_string()
        );
    }

    #[test]
    fn test_nested_tuple_value_renders_like_a_nested_tuple_expression() {
        let pat = parse_pat(quote!(((a, b), c)));
        let value = FakeCallValue::try_from(&pat).expect("nested tuple pattern should parse");

        assert_eq!(
            value.to_token_stream().to_string(),
            quote!(((a, b), c)).to_string()
        );
    }
}
