pub struct ImplItemFnInfo {
    pub struct_name: syn::Ident,
    pub method_name: syn::Ident,
    pub _param_types: Vec<syn::Type>,
    pub param_idents: Vec<syn::Ident>,
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

    /// The generic types of the struct and method are combined, in the order of struct types followed by method types.
    pub types: Vec<syn::Type>,
    /// The generic types of the struct.
    pub _struct_types: Vec<syn::Type>,
    /// The generic types of the method.
    pub _method_types: Vec<syn::Type>,

    /// The type IDs of the struct and method are combined, in the order of struct type IDs followed by method type IDs.
    pub type_ids: Vec<syn::Expr>,
    /// The type IDs of the struct.
    pub _struct_type_ids: Vec<syn::Expr>,
    /// The type IDs of the method.
    pub _method_type_ids: Vec<syn::Expr>,
}
