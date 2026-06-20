use quote::quote;

use crate::extract::function::FunctionGenericInfo;

/// Extracts the generic information from a `Generics` object, including the count of generic parameters, the generic type parameters themselves, their identifiers, and their corresponding `TypeId` expressions.
///
/// This is used for free functions and not impl blocks, as impl blocks require special handling to combine the generic parameters from both the struct and the method.
pub fn extract_generic_function_info(generics: &syn::Generics) -> Option<FunctionGenericInfo> {
    let count = generics.params.len();
    let type_params = extract_generic_type_params(generics);

    if type_params.is_empty() {
        return None;
    }

    let idents = extract_generic_idents_from_params(&type_params);
    let type_ids = build_type_id_array(&idents);

    Some(FunctionGenericInfo {
        count,
        type_params,
        idents,
        type_ids,
    })
}

/// Extract the generic type parameters (e.g. `T: Display + 'static`, `U: 'static`) from a impl block method.
///
/// The generics of the struct and method are combined, in the order of struct generics followed by method generics.
pub fn extract_generic_impl_info(
    item_impl: &syn::ItemImpl,
    method: &syn::ImplItemFn
) -> Option<FunctionGenericInfo> {
    if method.sig.generics.params.is_empty() && item_impl.generics.params.is_empty() {
        return None;
    }

    let struct_generic_params = extract_generic_type_params(&item_impl.generics);
    let fn_generic_params = extract_generic_type_params(&method.sig.generics);
    let generic_params = struct_generic_params
        .into_iter()
        .chain(fn_generic_params.into_iter())
        .collect::<Vec<_>>();

    let generic_idents = extract_generic_idents_from_params(&generic_params);
    let generic_type_ids = build_type_id_array(&generic_idents);

    Some(FunctionGenericInfo {
        count: generic_params.len(),
        type_params: generic_params,
        idents: generic_idents,
        type_ids: generic_type_ids,
    })
}

/// Extract the generic type parameters (e.g. `T: Display + 'static`, `U: 'static`) from a `Generics` object
///
/// Returns a vector of `TypeParam` objects representing the generic type parameters
pub fn extract_generic_type_params(generics: &syn::Generics) -> Vec<syn::TypeParam> {
    generics.params
        .iter()
        .filter_map(|param| {
            if let syn::GenericParam::Type(type_param) = param {
                Some(type_param.clone())
            } else {
                None
            }
        })
        .collect()
}

/// Extract the generic idents (e.g. `T`, `U`) from a list of generic parameters (e.g. `T: Display + 'static`, `U: 'static`)
pub fn extract_generic_idents_from_params(generic_params: &[syn::TypeParam]) -> Vec<syn::Ident> {
    generic_params
        .iter()
        .map(|param| param.ident.clone())
        .collect()
}

/// Build TypeId array: [TypeId::of::<T>(), TypeId::of::<U>(), ...]
pub fn build_type_id_array(generic_idents: &[syn::Ident]) -> Vec<syn::Expr> {
    generic_idents
        .iter()
        .map(|ident| {
            quote! { std::any::TypeId::of::<#ident>() }
        })
        .map(|ts| syn::parse2(ts))
        .collect::<syn::Result<_>>()
        .expect("Type ids must be parsable to an expression. This should not fail.")
}
