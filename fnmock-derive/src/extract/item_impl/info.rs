pub struct ImplItemFnInfo {
    pub struct_name: syn::Ident,
    pub method_name: syn::Ident,
    pub param_pats: Vec<syn::Pat>,
    pub fn_closure_trait: syn::TraitBound,
    pub generic_info: Option<ImplItemFnGenericInfo>,
}

pub struct ImplItemFnGenericInfo {
    pub count: usize,

    /// The type params of the struct and method are combined, in the order of struct type params followed by method type params.
    pub generic_params: Vec<syn::GenericParam>,
    /// The type and const params of the struct.
    pub _struct_generic_params: Vec<syn::GenericParam>,
    /// The type and const params of the method.
    pub method_generic_params: Vec<syn::GenericParam>,

    /// The generic idents of the struct and method are combined, in the order of struct idents followed by method idents.
    pub idents: Vec<syn::Ident>,
    /// The generic idents of the struct.
    pub _struct_idents: Vec<syn::Ident>,
    /// The generic idents of the method.
    pub _method_idents: Vec<syn::Ident>,

    /// The generic keys of the struct and method are combined, in the order of struct generic keys followed by method generic keys.
    pub generic_keys: Vec<syn::Expr>,
    /// The generic keys of the struct.
    pub _struct_generic_keys: Vec<syn::Expr>,
    /// The generic keys of the method.
    pub _method_generic_keys: Vec<syn::Expr>,
}
