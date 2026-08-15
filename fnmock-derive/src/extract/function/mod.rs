//! Extraction of the fake information for a free function.

use crate::{
    extract::{
        function::{generics::extract_generic_function_info, info::FunctionInfo},
        lifetimes::extract_lifetimes_from_generics,
        params::{extract_param_pats, extract_param_types},
    },
    names::NameType,
};

mod generics;
pub mod info;

/// Extracts the function information from a `syn::ItemFn`, including the function name, parameter types, parameter identifiers, function pointer type, and generic information if present.
///
/// `name_type` says which attribute is being expanded, so that a rejected construct is reported
/// against the attribute the user actually applied.
pub fn extract_function_info(
    item_fn: &syn::ItemFn,
    name_type: NameType,
) -> syn::Result<FunctionInfo> {
    if let Some(const_token) = &item_fn.sig.constness {
        return Err(syn::Error::new_spanned(
            const_token,
            format!(
                "The {} attribute does not support const fn. The code fnmock injects cannot run in a const context.",
                name_type.attribute_name()
            ),
        ));
    }

    let name = item_fn.sig.ident.clone();
    let visibility = item_fn.vis.clone();
    let params = item_fn.sig.inputs.iter().cloned().collect::<Vec<_>>();
    let param_types = extract_param_types(&params, None, name_type)?;
    let param_pats = extract_param_pats(&params);
    let generic_info = extract_generic_function_info(&item_fn.sig.generics)?;
    let lifetimes = extract_lifetimes_from_generics(&item_fn.sig.generics);

    Ok(FunctionInfo {
        name,
        visibility,
        param_pats,
        param_types,
        lifetimes,
        return_type: item_fn.sig.output.clone(),
        generic_info,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_fn_is_rejected() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            const fn foo(a: i32) -> i32 { a }
        };

        let result = extract_function_info(&item_fn, NameType::Fake);

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

        let result = extract_function_info(&item_fn, NameType::Fake);

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

        let result = extract_function_info(&item_fn, NameType::Fake);

        assert!(
            result.is_err(),
            "expected a free function with a `self` receiver to be rejected, not panic"
        );
    }
}
