pub mod function;
pub mod impl_block;

pub struct SpyScheme {
    pub store_name: syn::Ident,
    pub matcher_name: syn::Ident,
    /// The name of the wrapper struct `Params<'a>` is set to; see
    /// [`build_params_name`] for why it exists.
    pub params_name: syn::Ident,

    /// One identifier per parameter, in declaration order.
    pub param_idents: Vec<syn::Ident>,
    /// One type per parameter, in declaration order, with references stripped and lifetimes
    /// elided; see [`build_spy_params`].
    pub param_types: Vec<syn::Type>,
    /// One type per parameter, in declaration order, for the element type of the matcher's
    /// `Params<'a>` tuple: like `param_types`, but with any lifetime substituted for the tuple's
    /// own `'a` instead of elided; see [`build_spy_params`].
    pub params_tuple_types: Vec<syn::Type>,
    /// The expressions the injected call passes to `internal_record_call`, one per parameter, in
    /// declaration order; see [`build_spy_params`].
    pub reference_call_values: Vec<syn::Expr>,

    /// One expression per generic parameter, in declaration order, that renders it into the
    /// display name of an instantiation (e.g. `"i32"`, or `"5"` for a const generic's value). Only
    /// used when `common.generic_scheme` is `Some`.
    pub generic_display_fragments: Vec<syn::Expr>,

    /// Whether the matcher can offer `expect`'s `Predicate<..>`-based matching, alongside
    /// `expectf`.
    ///
    /// `false` when any parameter's type still names a lifetime after [`build_spy_params`] strips
    /// and elides what it can: eliding a lifetime by omission only actually works inside a
    /// `Fn(..) -> ..` trait's own argument list, which is where `expectf`'s closure parameter
    /// lives but `expect`'s `Predicate<..>` bound and the matcher's own fields are not. Only
    /// `expectf` is offered in that case.
    pub supports_expect: bool,
}
