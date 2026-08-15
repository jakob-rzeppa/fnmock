//! The information extracted from a free function's signature.

/// Everything the generators need to know about a fakeable free function.
pub struct FunctionInfo {
    /// The function's own name, which every generated name is derived from.
    pub name: syn::Ident,

    /// The function's visibility, which is copied to the generated items.
    pub visibility: syn::Visibility,

    /// The parameter patterns, in declaration order. Used to forward the call's arguments to the
    /// fake closure.
    pub param_pats: Vec<syn::Pat>,

    /// The parameter types, in declaration order and matching `param_pats`. A spy derives what it
    /// matches on from these; a fake only needs them by way of the `Fn(..) -> ..` trait bound
    /// built from these plus `lifetimes` and `return_type`.
    pub param_types: Vec<syn::Type>,

    /// The function's lifetime parameters. Only a fake needs these, to bind them higher-ranked on
    /// its closure trait.
    pub lifetimes: Vec<syn::Lifetime>,

    /// The function's return type.
    pub return_type: syn::ReturnType,

    /// The function's generic parameters, or `None` if it has none (lifetimes don't count — they
    /// are not part of the fake's key).
    pub generic_info: Option<FunctionGenericInfo>,
}

/// The generic parameters of a fakeable free function.
///
/// A generic function gets one fake per combination of generic arguments, so its parameters have
/// to be carried through to the store as key expressions.
pub struct FunctionGenericInfo {
    /// How many type and const parameters there are. Becomes the `GENERIC_COUNT` const generic of
    /// the generated `GenericFakeStore`.
    pub count: usize,

    /// The parameters including their bounds (e.g. `T: Display + 'static`), for redeclaring them
    /// on generated items.
    pub generic_params: Vec<syn::GenericParam>,

    /// Just the parameters' identifiers (e.g. `T`), for instantiating generated items.
    pub idents: Vec<syn::Ident>,

    /// The `GenericKeyPart` expressions that key this function's fake store, in parameter order.
    pub generic_keys: Vec<syn::Expr>,
}
