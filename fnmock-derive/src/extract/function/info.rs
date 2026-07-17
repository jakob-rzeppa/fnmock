pub struct FunctionInfo {
    pub name: syn::Ident,
    pub _param_types: Vec<syn::Type>,
    pub param_pats: Vec<syn::Pat>,
    pub fn_closure_trait: syn::TraitBound,
    pub generic_info: Option<FunctionGenericInfo>,
}

pub struct FunctionGenericInfo {
    pub count: usize,
    pub generic_params: Vec<syn::GenericParam>,
    pub idents: Vec<syn::Ident>,
    pub generic_keys: Vec<syn::Expr>,
}
