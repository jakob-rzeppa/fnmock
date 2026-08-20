//! The information extracted from a free function's signature.

use crate::item_info::{
    generic_param_info::{GenericParamInfo, extract_generic_param_infos},
    lifetimes::extract_lifetimes_from_generics,
    param_info::{ParamInfo, extract_params},
};

/// Everything the generators need to know about a fakeable free function.
pub struct FunctionInfo {
    /// The original, unmodified item.
    pub item_fn: syn::ItemFn,

    /// The function's own name, which every generated name is derived from.
    pub name: syn::Ident,

    /// The function's visibility, which is copied to the generated items.
    pub visibility: syn::Visibility,

    /// The parameters, in declaration order. Used to forward the call's arguments to the fake
    /// closure and, by way of their types plus `lifetimes` and `return_type`, to build the
    /// `Fn(..) -> ..` trait bound a fake must satisfy.
    pub params: Vec<ParamInfo>,

    /// The function's lifetime parameters. Only a fake needs these, to bind them higher-ranked on
    /// its closure trait.
    pub lifetimes: Vec<syn::Lifetime>,

    /// The function's return type.
    pub return_type: syn::ReturnType,

    /// The function's generic parameters. In the order they appear in the signature.
    pub generic_params: Vec<GenericParamInfo>,
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
        let fn_args = item_fn.sig.inputs.iter().cloned().collect::<Vec<_>>();
        let params = extract_params(&fn_args, None)?;
        let lifetimes = extract_lifetimes_from_generics(&item_fn.sig.generics);
        let return_type = item_fn.sig.output.clone();
        let generic_params = extract_generic_param_infos(&item_fn.sig.generics)?;

        Ok(FunctionInfo {
            item_fn,
            name,
            visibility,
            params,
            lifetimes,
            return_type,
            generic_params,
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
