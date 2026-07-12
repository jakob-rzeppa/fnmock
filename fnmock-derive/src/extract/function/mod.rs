use crate::extract::{
    fn_closure_trait::build_fn_closure_trait,
    function::{ generics::extract_generic_function_info, info::FunctionInfo },
    lifetimes::extract_lifetimes_from_generics,
    params::{ extract_param_pats, extract_param_types },
};

pub mod info;
mod generics;

/// Extracts the function information from a `syn::ItemFn`, including the function name, parameter types, parameter identifiers, function pointer type, and generic information if present.
pub fn extract_function_info(item_fn: &syn::ItemFn) -> syn::Result<FunctionInfo> {
    let name = item_fn.sig.ident.clone();
    let params = item_fn.sig.inputs.iter().cloned().collect::<Vec<_>>();
    let param_types = extract_param_types(&params, None);
    let param_pats = extract_param_pats(&params);
    let generic_info = extract_generic_function_info(&item_fn.sig.generics)?;
    let lifetimes = extract_lifetimes_from_generics(&item_fn.sig.generics);
    let fn_closure_trait = build_fn_closure_trait(&lifetimes, &param_types, &item_fn.sig.output)?;

    Ok(FunctionInfo {
        name,
        _param_types: param_types,
        param_pats,
        fn_closure_trait,
        generic_info,
    })
}
