pub struct SpyInlineCallInfo {
    /// The name of the generated module itself.
    ///
    /// Example: `get_user_spy_module`.
    pub module_name: syn::Ident,

    /// The param call values as references.
    /// Only non reference types are converted to references.
    /// "id: &str, name: String" becomes "id, &name"
    pub reference_call_values: Vec<syn::Expr>,
}
