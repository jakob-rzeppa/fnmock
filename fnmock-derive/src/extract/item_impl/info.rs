//! The information extracted from an impl block method's signature.

/// Everything the generators need to know about one fakeable method of an inherent impl block.
pub struct ImplItemFnInfo {
    /// The type the impl block is for. Combined with the method name to keep the generated names
    /// of two same-named methods on different types apart.
    pub struct_name: syn::Ident,

    /// The method's own name.
    pub method_name: syn::Ident,

    /// The parameter patterns, in declaration order, with the receiver represented as a plain
    /// `self` identifier. Used to forward the call's arguments to the fake closure.
    pub param_pats: Vec<syn::Pat>,

    /// The `Fn(..) -> ..` trait bound a fake for this method must satisfy. The receiver is its
    /// first argument, and every `Self` has been replaced by the impl block's concrete type.
    pub fn_closure_trait: syn::TraitBound,

    /// The generic parameters of the struct and method, or `None` if neither has any.
    pub generic_info: Option<ImplItemFnGenericInfo>,
}

/// The generic parameters in scope for a fakeable impl block method.
///
/// A method's fake is keyed by the struct's and the method's generic arguments together, since a
/// method on `Foo<i32>` and the same method on `Foo<u32>` need independent fakes. The struct-only
/// and method-only splits are kept alongside the combined lists because generated items need
/// different subsets: an access method, for instance, may only redeclare the method's own
/// parameters, as the struct's are already in scope from the enclosing `impl<..>` block.
pub struct ImplItemFnGenericInfo {
    /// How many type and const parameters there are in total. Becomes the `GENERIC_COUNT` const
    /// generic of the generated `GenericFakeStore`.
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
