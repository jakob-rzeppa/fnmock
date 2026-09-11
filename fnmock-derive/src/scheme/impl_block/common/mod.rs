use crate::{
    item_info::{generic_param_info::GenericParamInfo, original::OriginalImpl},
    scheme::common::generic_scheme::GenericScheme,
};

pub struct ImplCommonScheme {
    pub original: OriginalImpl,
}

pub struct ImplCommonMethodScheme {
    pub vis: syn::Visibility,

    pub accessor_name: syn::Ident,
    pub module_name: syn::Ident,
    pub display_name: String,
    pub interface_name: syn::Ident,

    /// The struct's and method's generics, combined.
    pub generic_scheme: Option<GenericScheme>,
    pub method_generic_params: Vec<syn::GenericParam>,
}

/// Merges the struct's generics (shared by every method in the impl block) with one method's own.
///
/// The struct's come first: `GenericScheme::keys`, which keys a method's store by the generics it
/// was instantiated with, and the spy's `generic_display_fragments` both read this sequence
/// positionally, so the order is load-bearing and has to be the same for every strategy.
///
/// Returns the method's own generic params alongside the merged infos, because only those get
/// redeclared on the generated accessor -- the struct's are already in scope from the enclosing
/// `impl<..>` block.
pub fn combine_generic_param_infos(
    struct_generic_param_infos: &[GenericParamInfo],
    method_generic_param_infos: Vec<GenericParamInfo>,
) -> (Vec<syn::GenericParam>, Vec<GenericParamInfo>) {
    let method_generic_params = method_generic_param_infos
        .iter()
        .map(|g| g.param.clone())
        .collect::<Vec<_>>();

    let combined = struct_generic_param_infos
        .iter()
        .cloned()
        .chain(method_generic_param_infos)
        .collect::<Vec<_>>();

    (method_generic_params, combined)
}
