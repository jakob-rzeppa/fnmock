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
                    if let Some(self_ty) = self_ty {
                        if receiver.reference.is_some() {
                            if receiver.mutability.is_some() {
                                Some(syn::parse_quote! { &mut #self_ty })
                            } else {
                                Some(syn::parse_quote! { &#self_ty })
                            }
                        } else {
                            Some(syn::parse_quote! { #self_ty })
                        }
                    } else {
                        unreachable!(
                            "Receiver found but no self type provided when extracting parameter types. This should not happen."
                        )
                    }
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
