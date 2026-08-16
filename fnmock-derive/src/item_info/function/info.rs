//! The information extracted from a free function's signature.

use crate::item_info::{
    function::generics::extract_generic_function_info,
    lifetimes::extract_lifetimes_from_generics,
    params::{extract_param_pats, extract_param_types},
};

/// Everything the generators need to know about a fakeable free function.
pub struct FunctionInfo {
    /// The original, unmodified item.
    pub item_fn: syn::ItemFn,

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

impl TryFrom<syn::ItemFn> for FunctionInfo {
    type Error = syn::Error;

    fn try_from(item_fn: syn::ItemFn) -> Result<Self, Self::Error> {
        if let Some(const_token) = &item_fn.sig.constness {
            return Err(syn::Error::new_spanned(
                const_token,
                format!(
                    "The macro does not support const fn. The code fnmock injects cannot run in a const context."
                ),
            ));
        }

        let name = item_fn.sig.ident.clone();
        let visibility = item_fn.vis.clone();
        let params = item_fn.sig.inputs.iter().cloned().collect::<Vec<_>>();
        let param_types = extract_param_types(&params, None)?;
        let param_pats = extract_param_pats(&params);
        let lifetimes = extract_lifetimes_from_generics(&item_fn.sig.generics);
        let return_type = item_fn.sig.output.clone();
        let generic_info = extract_generic_function_info(&item_fn.sig.generics)?;

        Ok(FunctionInfo {
            item_fn,
            name,
            visibility,
            param_pats,
            param_types,
            lifetimes,
            return_type,
            generic_info,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_fn_is_rejected() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            const fn foo(a: i32) -> i32 { a }
        };

        let result = FunctionInfo::try_from(item_fn);

        assert!(
            result.is_err(),
            "expected #[fakeable] on a const fn to be rejected"
        );
    }

    #[test]
    fn test_non_const_fn_is_accepted() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            fn foo(a: i32) -> i32 { a }
        };

        let result = FunctionInfo::try_from(item_fn);

        assert!(
            result.is_ok(),
            "expected #[fakeable] on a non-const fn to be accepted"
        );
    }

    #[test]
    fn test_free_function_with_self_receiver_is_rejected() {
        // syn parses `fn foo(self)` as a free `ItemFn` with a receiver even though rustc would
        // reject it. fnmock must surface a spanned error rather than panicking during expansion.
        let item_fn: syn::ItemFn = syn::parse_quote! {
            fn foo(self) {}
        };

        let result = FunctionInfo::try_from(item_fn);

        assert!(
            result.is_err(),
            "expected a free function with a `self` receiver to be rejected, not panic"
        );
    }
}
