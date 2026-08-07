//! The information needed to generate the injected call recording.

use crate::{
    extract::{
        function::info::FunctionInfo,
        params::{build_reference_call_value, extract_param_idents},
    },
    names::{NameType, build_module_name},
};

/// Everything the injected call recording needs to name the spy and hand it the call's arguments.
#[derive(Clone)]
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

impl TryFrom<&FunctionInfo> for SpyInlineCallInfo {
    type Error = syn::Error;

    fn try_from(function_info: &FunctionInfo) -> Result<Self, Self::Error> {
        let param_idents = extract_param_idents(&function_info.param_pats, NameType::Spy)?;

        Ok(SpyInlineCallInfo {
            module_name: build_module_name(&function_info.name, NameType::Spy),
            reference_call_values: param_idents
                .iter()
                .zip(&function_info.param_types)
                .map(|(ident, ty)| build_reference_call_value(ident, ty))
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;
    use crate::extract::function::extract_function_info;

    fn function_info(item_fn: syn::ItemFn) -> FunctionInfo {
        extract_function_info(&item_fn, NameType::Spy).expect("valid standalone function")
    }

    fn render(exprs: &[syn::Expr]) -> Vec<String> {
        exprs
            .iter()
            .map(|expr| expr.to_token_stream().to_string())
            .collect()
    }

    #[test]
    fn test_try_from_function_info_borrows_only_the_params_that_are_not_already_references() {
        let info = SpyInlineCallInfo::try_from(&function_info(syn::parse_quote! {
            fn get_user(mut id: String, uuid: &str) -> String {
                todo!()
            }
        }))
        .expect("conversion should succeed for a standalone function");

        assert_eq!(info.module_name.to_string(), "get_user_spy_module");
        assert_eq!(
            render(&info.reference_call_values),
            vec![
                quote::quote!(&id).to_string(),
                quote::quote!(uuid).to_string()
            ]
        );
    }

    /// A `&mut` parameter has to be reborrowed rather than forwarded, or recording the call would
    /// move it out of the binding and the rest of the user's body could no longer use it.
    #[test]
    fn test_try_from_function_info_reborrows_a_mutable_reference_param() {
        let info = SpyInlineCallInfo::try_from(&function_info(syn::parse_quote! {
            fn bump(count: &mut usize) {}
        }))
        .expect("conversion should succeed for a `&mut` parameter");

        assert_eq!(
            render(&info.reference_call_values),
            vec![quote::quote!(&*count).to_string()]
        );
    }

    #[test]
    fn test_try_from_function_info_zero_params() {
        let info = SpyInlineCallInfo::try_from(&function_info(syn::parse_quote! {
            fn ping() {}
        }))
        .expect("conversion should succeed for a function with no parameters");

        assert!(info.reference_call_values.is_empty());
    }

    #[test]
    fn test_try_from_function_info_rejects_a_destructuring_param() {
        let result = SpyInlineCallInfo::try_from(&function_info(syn::parse_quote! {
            fn foo((a, b): (i32, i32)) {}
        }));

        assert!(
            result.is_err(),
            "a destructuring parameter should be rejected"
        );
    }
}
