pub struct GenericScheme {
    /// The parameters including their bounds (e.g. `T: Display + 'static`), for redeclaring them
    /// on generated items.
    pub params: Vec<syn::GenericParam>,
    /// Just the parameters' identifiers (e.g. `T`), for instantiating generated items.
    pub idents: Vec<syn::Ident>,
    /// `idents` with const generics filtered out (they can't appear inside `PhantomData`).
    pub idents_without_const_generics: Vec<syn::Ident>,
    /// The `GenericKeyPart` expressions that key a store by these generics, in parameter order.
    pub keys: Vec<syn::Expr>,
}
