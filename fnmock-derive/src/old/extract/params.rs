//! Extraction of a function's parameter types and patterns.

use quote::quote;
use syn::{spanned::Spanned, visit_mut::VisitMut};

use crate::{
    old::extract::{call_value::CallValue, replace_self::ReplaceSelf},
    old::names::NameType,
};

/// Extracts the parameter types from a list of function parameters, replacing any `Self` types with the provided `self_ty`.
///
/// Pass `Some(self_ty)` for impl block methods and `None` for free functions. A `self` receiver
/// contributes its own type as the first parameter, so a fake for a method takes the receiver as
/// its first argument.
///
/// # Errors
///
/// Returns a spanned error if a `self` receiver appears while `self_ty` is `None`. syn parses
/// `fn foo(self)` as a free function even though rustc would reject it, so this has to be caught
/// rather than assumed away.
pub fn extract_param_types(
    params: &[syn::FnArg],
    self_ty: Option<&syn::Type>,
    name_type: NameType,
) -> syn::Result<Vec<syn::Type>> {
    // If a `self_ty` is provided, we will replace any `Self` types in the parameter types with the provided `self_ty`.
    // If no `self_ty` is provided, we don't need to replace `Self` in the parameter types, since we are in a standalone function context.
    let mut self_replacer = self_ty.map(ReplaceSelf::new);

    params
        .iter()
        .map(|param| {
            match param {
                syn::FnArg::Typed(pat_type) => {
                    let mut ty = pat_type.ty.as_ref().clone();

                    if let Some(replacer) = &mut self_replacer {
                        replacer.visit_type_mut(&mut ty);
                    }
                    Ok(ty)
                }
                syn::FnArg::Receiver(receiver) => {
                    if self_ty.is_none() {
                        return Err(
                            syn::Error::new_spanned(
                                receiver,
                                format!(
                                    "The `{}` attribute found a `self` receiver on a free function. `self` receivers are only supported on methods inside an inherent impl block.",
                                    name_type.attribute_name()
                                )
                            )
                        );
                    }
                    // `receiver.ty` already holds the receiver's full type for every form syn can
                    // parse - `&Self`, `&mut Self`, `Self`, and explicit forms like `Box<Self>` or
                    // `Rc<Self>` - including any named self-lifetime, so we don't need to
                    // reconstruct it by hand from `reference`/`mutability`.
                    let mut ty = receiver.ty.as_ref().clone();
                    if let Some(replacer) = &mut self_replacer {
                        replacer.visit_type_mut(&mut ty);
                    }
                    Ok(ty)
                }
            }
        })
        .collect()
}

/// Extracts the parameter patterns / identifiers from a list of function parameters.
///
/// These become the arguments the injected inline call forwards to the fake, so a `self` receiver
/// is represented as a plain `self` identifier and mutability in a pattern (`mut x`) is kept as
/// written — it is part of the binding, not of the value being passed on.
///
/// Patterns that cannot be forwarded are rejected later, when the patterns are turned into call
/// arguments; see [`CallValue`].
pub fn extract_param_pats(params: &[syn::FnArg]) -> Vec<syn::Pat> {
    params
        .iter()
        .map(|param| match param {
            syn::FnArg::Typed(pat_type) => pat_type.pat.as_ref().clone(),
            syn::FnArg::Receiver(_) => syn::Pat::Ident(syn::PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: syn::Ident::new(
                    "self",
                    params
                        .first()
                        .map(|p| p.span())
                        .unwrap_or_else(proc_macro2::Span::call_site),
                ),
                subpat: None,
            }),
        })
        .collect()
}

/// Extracts one identifier per parameter, in declaration order.
///
/// A spy names each parameter in its matcher — as a struct field, and in the message a failed
/// expectation prints — so unlike a fake it needs a name for every parameter, not just a way to
/// forward its value.
///
/// # Errors
///
/// Returns a spanned error for any pattern that binds no single name, either because [`CallValue`]
/// rejects it outright (`ref`, wildcards, struct destructuring) or because it destructures into
/// several bindings (`(a, b): (i32, i32)`).
pub fn extract_param_idents(
    param_pats: &[syn::Pat],
    name_type: NameType,
) -> syn::Result<Vec<syn::Ident>> {
    param_pats
        .iter()
        .map(|pat| match CallValue::try_from(pat)? {
            CallValue::Ident(ident) => Ok(ident),
            CallValue::Tuple(_) | CallValue::Slice(_) => Err(syn::Error::new_spanned(
                pat,
                format!(
                    "The `{}` attribute only supports plain identifier parameters. This parameter destructures its value, so there is no name to match it under.",
                    name_type.attribute_name()
                ),
            )),
        })
        .collect()
}

/// Strips one level of reference from a type: `&T` and `&mut T` both become `T`, anything else is
/// returned unchanged.
///
/// A spy observes every argument by shared reference, so `fn get_user(id: String, uuid: &str)` is
/// matched as `(&String, &str)`. Getting there means stripping what the user already wrote as a
/// reference, so that `&str` does not turn into `&&str`.
pub fn strip_reference(ty: &syn::Type) -> syn::Type {
    match ty {
        syn::Type::Reference(type_reference) => type_reference.elem.as_ref().clone(),
        other => other.clone(),
    }
}

/// Builds the expression that passes a parameter on by shared reference, so that its type lines up
/// with [`strip_reference`]'s.
///
/// - `&T`: forwarded as-is (`id`) — a shared reference is `Copy`, so the binding stays usable.
/// - `&mut T`: reborrowed (`&*id`). Forwarding it as-is would *move* the `&mut` out of the
///   binding and leave the rest of the user's body unable to use the parameter.
/// - anything else: borrowed (`&id`).
pub fn build_reference_call_value(ident: &syn::Ident, ty: &syn::Type) -> syn::Expr {
    let expr = match ty {
        syn::Type::Reference(type_reference) if type_reference.mutability.is_some() => {
            quote! { &*#ident }
        }
        syn::Type::Reference(_) => quote! { #ident },
        _ => quote! { &#ident },
    };

    syn::parse_quote!(#expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    mod extract_param_types {
        use super::*;

        fn param_type_string(params: &[syn::FnArg], self_ty: &syn::Type) -> String {
            let param_types = extract_param_types(params, Some(self_ty), NameType::Fake)
                .expect("extract_param_types should succeed for a receiver with a self type");
            assert_eq!(param_types.len(), 1);
            param_types[0].to_token_stream().to_string()
        }

        #[test]
        fn test_receiver_without_self_type_returns_error() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! {
                self
            }];

            let result = extract_param_types(&params, None, NameType::Fake);

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

            let param_types = extract_param_types(&params, None, NameType::Fake)
                .expect("extract_param_types should succeed for typed params with no self type");

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

            let param_types = extract_param_types(&params, Some(&self_ty), NameType::Fake).expect(
                "extract_param_types should succeed for a receiver followed by typed params",
            );

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

            let param_types = extract_param_types(&params, None, NameType::Fake).expect(
                "extract_param_types should succeed for a variety of typed params with no self type"
            );

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

            let param_types = extract_param_types(&params, Some(&self_ty), NameType::Fake).expect(
                "extract_param_types should succeed for a non-receiver param typed as `Self`",
            );

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

            let param_types = extract_param_types(&params, Some(&self_ty), NameType::Fake).expect(
                "extract_param_types should succeed for non-receiver params with `Self` nested inside other types"
            );

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

            let param_types = extract_param_types(&params, Some(&self_ty), NameType::Fake).expect(
                "extract_param_types should succeed for a non-receiver `Self` param with a generic struct self type"
            );

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

            let param_types = extract_param_types(&params, Some(&self_ty), NameType::Fake).expect(
                "extract_param_types should succeed for `Self` nested inside another type with a generic struct self type"
            );

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

            let param_types = extract_param_types(&params, Some(&self_ty), NameType::Fake).expect(
                "extract_param_types should succeed for a receiver plus a non-receiver `Self` param"
            );

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

    mod extract_param_pats {
        use super::*;

        fn pat_strings(params: &[syn::FnArg]) -> Vec<String> {
            extract_param_pats(params)
                .iter()
                .map(|pat| pat.to_token_stream().to_string())
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

    mod extract_param_idents {
        use super::*;

        fn ident_strings(params: &[syn::FnArg]) -> syn::Result<Vec<String>> {
            Ok(
                extract_param_idents(&extract_param_pats(params), NameType::Spy)?
                    .iter()
                    .map(|ident| ident.to_string())
                    .collect(),
            )
        }

        #[test]
        fn test_empty_params_returns_empty_vec() {
            let params: Vec<syn::FnArg> = vec![];

            assert!(
                ident_strings(&params)
                    .expect("a function with no parameters should be accepted")
                    .is_empty()
            );
        }

        #[test]
        fn test_multiple_params_preserve_order_and_drop_mutability() {
            let params: Vec<syn::FnArg> = vec![
                syn::parse_quote! { mut id: String },
                syn::parse_quote! { uuid: &str },
            ];

            assert_eq!(
                ident_strings(&params).expect("plain identifier parameters should be accepted"),
                vec!["id".to_string(), "uuid".to_string()]
            );
        }

        #[test]
        fn test_receiver_is_named_self() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! {
                &self
            }];

            assert_eq!(
                ident_strings(&params).expect("a receiver is a plain identifier pattern"),
                vec!["self".to_string()]
            );
        }

        #[test]
        fn test_tuple_destructuring_param_is_rejected() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { (a, b): (i32, i32) }];

            let Err(error) = ident_strings(&params) else {
                panic!("a destructuring parameter should be rejected: it binds no single name");
            };
            assert!(
                error.to_string().contains("#[spyable]"),
                "the error should name the attribute that was applied, got: {error}"
            );
        }

        #[test]
        fn test_slice_destructuring_param_is_rejected() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { [a, b]: [i32; 2] }];

            assert!(
                ident_strings(&params).is_err(),
                "a slice destructuring parameter should be rejected"
            );
        }

        #[test]
        fn test_pattern_call_value_already_rejects_is_still_rejected() {
            let params: Vec<syn::FnArg> = vec![syn::parse_quote! { ref a: i32 }];

            let Err(error) = ident_strings(&params) else {
                panic!("a `ref` parameter should be rejected");
            };
            assert!(
                error.to_string().to_lowercase().contains("ref"),
                "the error should mention `ref`, got: {error}"
            );
        }
    }

    mod strip_reference {
        use super::*;

        fn stripped(ty: syn::Type) -> String {
            strip_reference(&ty).to_token_stream().to_string()
        }

        #[test]
        fn test_shared_reference_loses_its_reference() {
            assert_eq!(stripped(syn::parse_quote!(&str)), quote!(str).to_string());
        }

        #[test]
        fn test_mutable_reference_loses_both_reference_and_mutability() {
            assert_eq!(
                stripped(syn::parse_quote!(&mut i32)),
                quote!(i32).to_string()
            );
        }

        #[test]
        fn test_named_lifetime_reference_loses_its_lifetime_with_the_reference() {
            assert_eq!(
                stripped(syn::parse_quote!(&'a String)),
                quote!(String).to_string()
            );
        }

        #[test]
        fn test_owned_type_is_unchanged() {
            assert_eq!(
                stripped(syn::parse_quote!(String)),
                quote!(String).to_string()
            );
        }

        #[test]
        fn test_only_the_outermost_reference_is_stripped() {
            assert_eq!(stripped(syn::parse_quote!(&&str)), quote!(&str).to_string());
        }

        #[test]
        fn test_reference_nested_inside_another_type_is_kept() {
            assert_eq!(
                stripped(syn::parse_quote!(Vec<&str>)),
                quote!(Vec<&str>).to_string()
            );
        }
    }

    mod build_reference_call_value {
        use super::*;

        fn call_value(ty: syn::Type) -> String {
            let ident = syn::Ident::new("id", proc_macro2::Span::call_site());
            build_reference_call_value(&ident, &ty)
                .to_token_stream()
                .to_string()
        }

        #[test]
        fn test_owned_param_is_borrowed() {
            assert_eq!(
                call_value(syn::parse_quote!(String)),
                quote!(&id).to_string()
            );
        }

        #[test]
        fn test_shared_reference_param_is_forwarded_as_is() {
            assert_eq!(call_value(syn::parse_quote!(&str)), quote!(id).to_string());
        }

        /// Forwarding a `&mut` binding as-is would move it, leaving the rest of the user's body
        /// unable to use the parameter, so it has to be reborrowed.
        #[test]
        fn test_mutable_reference_param_is_reborrowed() {
            assert_eq!(
                call_value(syn::parse_quote!(&mut String)),
                quote!(&*id).to_string()
            );
        }
    }
}
