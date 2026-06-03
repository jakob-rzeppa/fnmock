pub fn extract_param_idents(inputs: &[syn::FnArg]) -> Vec<syn::Ident> {
    inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    return Some(pat_ident.ident.clone());
                }
            }
            None
        })
        .collect()
}

pub fn extract_param_types(inputs: &[syn::FnArg]) -> Vec<syn::Type> {
    inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                return Some((*pat_type.ty).clone());
            }
            None
        })
        .collect()
}
