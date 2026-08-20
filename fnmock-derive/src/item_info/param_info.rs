use syn::{spanned::Spanned, visit_mut::VisitMut};

use crate::item_info::replace_self::ReplaceSelf;

/// Information about a single function parameter.
pub struct ParamInfo {
    pub pat: syn::Pat,

    /// The parameter's type, with any `Self` replaced by the provided `self_ty`.
    pub ty: syn::Type,
}

/// Extracts each parameter's pattern and type from a list of function parameters, replacing any
/// `Self` types with the provided `self_ty`.
///
/// Pass `Some(self_ty)` for impl block methods and `None` for free functions. A `self` receiver
/// contributes its own pattern and type as the first parameter, so a fake for a method takes the
/// receiver as its first argument.
///
/// # Errors
///
/// Returns a spanned error if a `self` receiver appears while `self_ty` is `None`. syn parses
/// `fn foo(self)` as a free function even though rustc would reject it, so this has to be caught
/// rather than assumed away.
pub fn extract_params(
    params: &[syn::FnArg],
    self_ty: Option<&syn::Type>,
) -> syn::Result<Vec<ParamInfo>> {
    // If a `self_ty` is provided, we will replace any `Self` types in the parameter types with the provided `self_ty`.
    // If no `self_ty` is provided, we don't need to replace `Self` in the parameter types, since we are in a standalone function context.
    let mut self_replacer = self_ty.map(ReplaceSelf::new);

    params
        .iter()
        .map(|param| {
            match param {
                syn::FnArg::Typed(pat_type) => {
                    let pat = pat_type.pat.as_ref().clone();
                    let mut ty = pat_type.ty.as_ref().clone();

                    if let Some(replacer) = &mut self_replacer {
                        replacer.visit_type_mut(&mut ty);
                    }
                    Ok(ParamInfo { pat, ty })
                }
                syn::FnArg::Receiver(receiver) => {
                    if self_ty.is_none() {
                        return Err(
                            syn::Error::new_spanned(
                                receiver,
                                format!(
                                    "The macro found a `self` receiver on a free function. `self` receivers are only supported on methods inside an inherent impl block."
                                )
                            )
                        );
                    }
                    let pat = syn::Pat::Ident(syn::PatIdent {
                        attrs: Vec::new(),
                        by_ref: None,
                        mutability: None,
                        ident: syn::Ident::new("self", receiver.span()),
                        subpat: None,
                    });
                    // `receiver.ty` already holds the receiver's full type for every form syn can
                    // parse - `&Self`, `&mut Self`, `Self`, and explicit forms like `Box<Self>` or
                    // `Rc<Self>` - including any named self-lifetime, so we don't need to
                    // reconstruct it by hand from `reference`/`mutability`.
                    let mut ty = receiver.ty.as_ref().clone();
                    if let Some(replacer) = &mut self_replacer {
                        replacer.visit_type_mut(&mut ty);
                    }
                    Ok(ParamInfo { pat, ty })
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    mod extract_params_types {
        use super::*;

        fn param_types(params: &[syn::FnArg], self_ty: Option<&syn::Type>) -> Vec<syn::Type> {
            extract_params(params, self_ty)
                .expect("extract_params should succeed")
                .into_iter()
                .map(|p| p.ty)
                .collect()
        }

        fn param_type_string(params: &[syn::FnArg], self_ty: &syn::Type) -> String {
            let param_types = param_types(params, Some(self_ty));
            assert_eq!(param_types.len(), 1);
            param_types[0].to_token_stream().to_string()
        }

        #[test]
        fn test_receiver_without_self_type_returns_error() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! {
                self
            }];

            let result = extract_params(&params, None);

            assert!(
                result.is_err(),
                "expected a `self` receiver with no self type (free function) to error, not panic"
            );
        }

        #[test]
        fn test_boxed_self_receiver_uses_actual_self_type() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { self: Box<Self> }];

            assert_eq!(
                param_type_string(&params, &self_ty),
                quote::quote!(Box<MyStruct>).to_string()
            );
        }

        #[test]
        fn test_named_lifetime_self_receiver_preserves_lifetime() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { &'a self }];

            assert_eq!(
                param_type_string(&params, &self_ty),
                quote::quote!(&'a MyStruct).to_string()
            );
        }

        #[test]
        fn test_ref_self_receiver_still_produces_reference_type() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! {
                &self
            }];

            assert_eq!(
                param_type_string(&params, &self_ty),
                quote::quote!(&MyStruct).to_string()
            );
        }

        #[test]
        fn test_mut_ref_self_receiver_still_produces_mut_reference_type() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! {
                &mut self
            }];

            assert_eq!(
                param_type_string(&params, &self_ty),
                quote::quote!(&mut MyStruct).to_string()
            );
        }

        #[test]
        fn test_by_value_self_receiver_still_produces_bare_type() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! {
                self
            }];

            assert_eq!(
                param_type_string(&params, &self_ty),
                quote::quote!(MyStruct).to_string()
            );
        }

        #[test]
        fn test_rc_self_receiver_uses_actual_self_type() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { self: Rc<Self> }];

            assert_eq!(
                param_type_string(&params, &self_ty),
                quote::quote!(Rc<MyStruct>).to_string()
            );
        }

        #[test]
        fn test_pin_mut_self_receiver_uses_actual_self_type() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { self: Pin<&mut Self> }];

            assert_eq!(
                param_type_string(&params, &self_ty),
                quote::quote!(Pin<&mut MyStruct>).to_string()
            );
        }

        #[test]
        fn test_explicit_self_type_receiver_uses_actual_self_type() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { self: Self }];

            assert_eq!(
                param_type_string(&params, &self_ty),
                quote::quote!(MyStruct).to_string()
            );
        }

        #[test]
        fn test_multiple_typed_params_preserves_order_and_types() {
            let params: Vec<syn::FnArg> = vec![
                syn::parse_quote! { a: i32 },
                syn::parse_quote! { b: String },
                syn::parse_quote! { c: bool },
            ];

            let param_types = param_types(&params, None);

            let type_strings: Vec<String> = param_types
                .iter()
                .map(|ty| ty.to_token_stream().to_string())
                .collect();
            assert_eq!(
                type_strings,
                vec![
                    quote::quote!(i32).to_string(),
                    quote::quote!(String).to_string(),
                    quote::quote!(bool).to_string()
                ]
            );
        }

        #[test]
        fn test_receiver_plus_multiple_typed_params_preserves_order() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct
            };
            let params: Vec<syn::FnArg> = vec![
                syn::parse_quote! {
                    &self
                },
                syn::parse_quote! { a: i32 },
                syn::parse_quote! { b: Vec<String> },
            ];

            let param_types = param_types(&params, Some(&self_ty));

            let type_strings: Vec<String> = param_types
                .iter()
                .map(|ty| ty.to_token_stream().to_string())
                .collect();
            assert_eq!(
                type_strings,
                vec![
                    quote::quote!(&MyStruct).to_string(),
                    quote::quote!(i32).to_string(),
                    quote::quote!(Vec<String>).to_string()
                ]
            );
        }

        #[test]
        fn test_various_param_types_are_preserved_unchanged() {
            let params: Vec<syn::FnArg> = vec![
                syn::parse_quote! { a: &str },
                syn::parse_quote! { b: &mut i32 },
                syn::parse_quote! { c: Option<Vec<u8>> },
                syn::parse_quote! { d: (i32, String) },
                syn::parse_quote! { e: *const u8 },
                syn::parse_quote! { f: fn(i32) -> bool },
            ];

            let param_types = param_types(&params, None);

            let type_strings: Vec<String> = param_types
                .iter()
                .map(|ty| ty.to_token_stream().to_string())
                .collect();

            #[rustfmt::skip]
            assert_eq!(
                type_strings,
                vec![
                    quote::quote!(&str).to_string(),
                    quote::quote!(&mut i32).to_string(),
                    quote::quote!(Option< Vec<u8> >).to_string(),
                    quote::quote!((i32, String)).to_string(),
                    quote::quote!(*const u8).to_string(),
                    quote::quote!(fn(i32) -> bool).to_string()
                ]
            );
        }

        #[test]
        fn test_self_in_non_receiver_param_type_is_replaced() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { other: Self }];

            let param_types = param_types(&params, Some(&self_ty));

            assert_eq!(
                param_types[0].to_token_stream().to_string(),
                quote::quote!(MyStruct).to_string()
            );
        }

        #[test]
        fn test_self_nested_in_non_receiver_param_type_is_replaced() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct
            };
            let params: Vec<syn::FnArg> = vec![
                syn::parse_quote! { others: Vec<Self> },
                syn::parse_quote! { pair: (Self, Self) },
            ];

            let param_types = param_types(&params, Some(&self_ty));

            assert_eq!(
                param_types[0].to_token_stream().to_string(),
                quote::quote!(Vec<MyStruct>).to_string()
            );
            assert_eq!(
                param_types[1].to_token_stream().to_string(),
                quote::quote!((MyStruct, MyStruct)).to_string()
            );
        }

        #[test]
        fn test_ref_self_receiver_with_generic_struct_self_type() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct<T>
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! {
                &self
            }];

            assert_eq!(
                param_type_string(&params, &self_ty),
                quote::quote!(&MyStruct<T>).to_string()
            );
        }

        #[test]
        fn test_boxed_self_receiver_with_generic_struct_self_type() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct<T>
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { self: Box<Self> }];

            #[rustfmt::skip]
            assert_eq!(
                param_type_string(&params, &self_ty),
                quote::quote!(Box<MyStruct<T> >).to_string()
            );
        }

        #[test]
        fn test_non_receiver_self_param_with_generic_struct_self_type() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct<T>
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { other: Self }];

            let param_types = param_types(&params, Some(&self_ty));

            assert_eq!(
                param_types[0].to_token_stream().to_string(),
                quote::quote!(MyStruct<T>).to_string()
            );
        }

        #[test]
        fn test_self_nested_in_param_with_generic_struct_self_type() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct<T>
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { others: Vec<Self> }];

            let param_types = param_types(&params, Some(&self_ty));

            #[rustfmt::skip]
            assert_eq!(
                param_types[0].to_token_stream().to_string(),
                quote::quote!(Vec< MyStruct<T> >).to_string()
            );
        }

        #[test]
        fn test_multiple_type_params_self_type_is_preserved_in_full() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct<K, V>
            };
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! {
                &mut self
            }];

            assert_eq!(
                param_type_string(&params, &self_ty),
                quote::quote!(&mut MyStruct<K, V>).to_string()
            );
        }

        #[test]
        fn test_receiver_and_non_receiver_self_params_are_both_replaced() {
            let self_ty: syn::Type = syn::parse_quote! {
                MyStruct
            };
            let params: Vec<syn::FnArg> = vec![
                syn::parse_quote! {
                    &self
                },
                syn::parse_quote! { other: Self },
            ];

            let param_types = param_types(&params, Some(&self_ty));

            assert_eq!(param_types.len(), 2);
            assert_eq!(
                param_types[0].to_token_stream().to_string(),
                quote::quote!(&MyStruct).to_string()
            );
            assert_eq!(
                param_types[1].to_token_stream().to_string(),
                quote::quote!(MyStruct).to_string()
            );
        }
    }

    mod extract_params_pats {
        use super::*;

        fn pat_strings(params: &[syn::FnArg]) -> Vec<String> {
            // A dummy self type, since these tests only check the extracted patterns; a receiver
            // needs `Some(self_ty)` to be accepted at all.
            let self_ty: syn::Type = syn::parse_quote!(MyStruct);
            extract_params(params, Some(&self_ty))
                .expect("extract_params should succeed")
                .iter()
                .map(|p| p.pat.to_token_stream().to_string())
                .collect()
        }

        #[test]
        fn test_empty_params_returns_empty_vec() {
            let params: Vec<syn::FnArg> = vec![];

            assert!(pat_strings(&params).is_empty());
        }

        #[test]
        fn test_single_typed_param_returns_its_ident() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { a: i32 }];

            assert_eq!(pat_strings(&params), vec![quote::quote!(a).to_string()]);
        }

        #[test]
        fn test_multiple_typed_params_preserves_order() {
            let params: Vec<syn::FnArg> = vec![
                syn::parse_quote! { a: i32 },
                syn::parse_quote! { b: String },
                syn::parse_quote! { c: bool },
            ];

            assert_eq!(
                pat_strings(&params),
                vec![
                    quote::quote!(a).to_string(),
                    quote::quote!(b).to_string(),
                    quote::quote!(c).to_string()
                ]
            );
        }

        #[test]
        fn test_by_value_self_receiver_returns_self_ident() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! {
                self
            }];

            assert_eq!(pat_strings(&params), vec![quote::quote!(self).to_string()]);
        }

        #[test]
        fn test_ref_self_receiver_returns_self_ident() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! {
                &self
            }];

            assert_eq!(pat_strings(&params), vec![quote::quote!(self).to_string()]);
        }

        #[test]
        fn test_boxed_self_receiver_returns_self_ident() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { self: Box<Self> }];

            assert_eq!(pat_strings(&params), vec![quote::quote!(self).to_string()]);
        }

        #[test]
        fn test_receiver_plus_multiple_typed_params_preserves_order() {
            let params: Vec<syn::FnArg> = vec![
                syn::parse_quote! {
                    &self
                },
                syn::parse_quote! { a: i32 },
                syn::parse_quote! { b: Vec<String> },
            ];

            assert_eq!(
                pat_strings(&params),
                vec![
                    quote::quote!(self).to_string(),
                    quote::quote!(a).to_string(),
                    quote::quote!(b).to_string()
                ]
            );
        }

        #[test]
        fn test_mut_ident_pattern_preserves_mutability() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { mut a: i32 }];

            assert_eq!(pat_strings(&params), vec![quote::quote!(mut a).to_string()]);
        }

        #[test]
        fn test_underscore_pattern_is_preserved() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { _: i32 }];

            assert_eq!(pat_strings(&params), vec![quote::quote!(_).to_string()]);
        }

        #[test]
        fn test_tuple_pattern_is_preserved() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { (a, b): (i32, i32) }];

            assert_eq!(
                pat_strings(&params),
                vec![quote::quote!((a, b)).to_string()]
            );
        }

        #[test]
        fn test_struct_destructure_pattern_is_preserved() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { MyStruct { a, b }: MyStruct }];

            assert_eq!(
                pat_strings(&params),
                vec![quote::quote!(MyStruct { a, b }).to_string()]
            );
        }

        #[test]
        fn test_reference_type_param_returns_bare_ident_pattern() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { a: &str }];

            assert_eq!(pat_strings(&params), vec![quote::quote!(a).to_string()]);
        }
    }
}
