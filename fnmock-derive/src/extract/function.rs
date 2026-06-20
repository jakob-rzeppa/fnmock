use crate::extract::{
    fn_ptr_type::build_fn_ptr_type,
    generic::extract_generic_function_info,
    params::{ extract_param_idents, extract_param_types },
};

pub struct FunctionInfo {
    pub name: syn::Ident,
    pub _param_types: Vec<syn::Type>,
    pub param_idents: Vec<syn::Ident>,
    pub fn_ptr_type: syn::Type,
    pub generic_info: Option<FunctionGenericInfo>,
}

pub struct FunctionGenericInfo {
    pub count: usize,
    pub type_params: Vec<syn::TypeParam>,
    pub idents: Vec<syn::Ident>,
    pub type_ids: Vec<syn::Expr>,
}

/// Extracts the function information from a `syn::ItemFn`, including the function name, parameter types, parameter identifiers, function pointer type, and generic information if present.
pub fn extract_function_info(item_fn: &syn::ItemFn) -> syn::Result<FunctionInfo> {
    let name = item_fn.sig.ident.clone();
    let params = item_fn.sig.inputs.iter().cloned().collect::<Vec<_>>();
    let param_types = extract_param_types(&params, None);
    let param_idents = extract_param_idents(&params);
    let fn_ptr_type = build_fn_ptr_type(&param_types, &item_fn.sig.output)?;
    let generic_info = extract_generic_function_info(&item_fn.sig.generics);

    Ok(FunctionInfo {
        name,
        _param_types: param_types,
        param_idents,
        fn_ptr_type,
        generic_info,
    })
}
