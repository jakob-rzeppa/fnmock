use syn::{ spanned::Spanned, visit_mut::VisitMut };

use crate::extract::replace_self::ReplaceSelf;

/// Extracts the parameter types from a list of function parameters, replacing any `Self` types with the provided `self_ty`.
pub fn extract_param_types(params: &[syn::FnArg], self_ty: Option<&syn::Type>) -> Vec<syn::Type> {
    let mut self_replacer = self_ty.map(|ty| ReplaceSelf::new(ty));

    params
        .iter()
        .filter_map(|param| {
            match param {
                syn::FnArg::Typed(pat_type) => {
                    let mut ty = pat_type.ty.as_ref().clone();
                    if let Some(replacer) = &mut self_replacer {
                        replacer.visit_type_mut(&mut ty);
                    }
                    Some(ty)
                }
                syn::FnArg::Receiver(receiver) => {
                    if self_ty.is_none() {
                        unreachable!(
                            "Receiver found but no self type provided when extracting parameter types. This should not happen."
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
                    Some(ty)
                }
            }
        })
        .collect()
}

/// Extracts the parameter patterns / identifiers from a list of function parameters.
pub fn extract_param_pats(params: &[syn::FnArg]) -> Vec<syn::Pat> {
    params
        .iter()
        .filter_map(|param| {
            match param {
                syn::FnArg::Typed(pat_type) => { Some(pat_type.pat.as_ref().clone()) }
                syn::FnArg::Receiver(_) =>
                    Some(
                        syn::Pat::Ident(syn::PatIdent {
                            attrs: Vec::new(),
                            by_ref: None,
                            mutability: None,
                            ident: syn::Ident::new(
                                "self",
                                params
                                    .first()
                                    .map(|p| p.span())
                                    .unwrap_or_else(proc_macro2::Span::call_site)
                            ),
                            subpat: None,
                        })
                    ),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    fn param_type_string(params: &[syn::FnArg], self_ty: &syn::Type) -> String {
        let param_types = extract_param_types(params, Some(self_ty));
        assert_eq!(param_types.len(), 1);
        param_types[0].to_token_stream().to_string()
    }

    #[test]
    fn test_boxed_self_receiver_uses_actual_self_type() {
        let self_ty: syn::Type = syn::parse_quote! { MyStruct };
        let params: Vec<syn::FnArg> = vec![syn::parse_quote! { self: Box<Self> }];

        assert_eq!(param_type_string(&params, &self_ty), quote::quote!(Box<MyStruct>).to_string());
    }

    #[test]
    fn test_named_lifetime_self_receiver_preserves_lifetime() {
        let self_ty: syn::Type = syn::parse_quote! { MyStruct };
        let params: Vec<syn::FnArg> = vec![syn::parse_quote! { &'a self }];

        assert_eq!(param_type_string(&params, &self_ty), quote::quote!(&'a MyStruct).to_string());
    }

    #[test]
    fn test_ref_self_receiver_still_produces_reference_type() {
        let self_ty: syn::Type = syn::parse_quote! { MyStruct };
        let params: Vec<syn::FnArg> = vec![syn::parse_quote! { &self }];

        assert_eq!(param_type_string(&params, &self_ty), quote::quote!(&MyStruct).to_string());
    }

    #[test]
    fn test_mut_ref_self_receiver_still_produces_mut_reference_type() {
        let self_ty: syn::Type = syn::parse_quote! { MyStruct };
        let params: Vec<syn::FnArg> = vec![syn::parse_quote! { &mut self }];

        assert_eq!(param_type_string(&params, &self_ty), quote::quote!(&mut MyStruct).to_string());
    }

    #[test]
    fn test_by_value_self_receiver_still_produces_bare_type() {
        let self_ty: syn::Type = syn::parse_quote! { MyStruct };
        let params: Vec<syn::FnArg> = vec![syn::parse_quote! { self }];

        assert_eq!(param_type_string(&params, &self_ty), quote::quote!(MyStruct).to_string());
    }

    #[test]
    fn test_rc_self_receiver_uses_actual_self_type() {
        let self_ty: syn::Type = syn::parse_quote! { MyStruct };
        let params: Vec<syn::FnArg> = vec![syn::parse_quote! { self: Rc<Self> }];

        assert_eq!(param_type_string(&params, &self_ty), quote::quote!(Rc<MyStruct>).to_string());
    }

    #[test]
    fn test_pin_mut_self_receiver_uses_actual_self_type() {
        let self_ty: syn::Type = syn::parse_quote! { MyStruct };
        let params: Vec<syn::FnArg> = vec![syn::parse_quote! { self: Pin<&mut Self> }];

        assert_eq!(param_type_string(&params, &self_ty), quote::quote!(Pin<&mut MyStruct>).to_string());
    }

    #[test]
    fn test_explicit_self_type_receiver_uses_actual_self_type() {
        let self_ty: syn::Type = syn::parse_quote! { MyStruct };
        let params: Vec<syn::FnArg> = vec![syn::parse_quote! { self: Self }];

        assert_eq!(param_type_string(&params, &self_ty), quote::quote!(MyStruct).to_string());
    }
}
