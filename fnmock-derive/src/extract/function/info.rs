pub struct FunctionInfo {
    pub name: syn::Ident,
    pub _param_types: Vec<syn::Type>,
    pub param_idents: Vec<syn::Ident>,
    pub fn_closure_trait: syn::TraitBound,
    pub generic_info: Option<FunctionGenericInfo>,
}

pub struct FunctionGenericInfo {
    pub count: usize,
    pub type_params: Vec<syn::TypeParam>,
    pub types: Vec<syn::Type>,
    pub type_ids: Vec<syn::Expr>,
}
