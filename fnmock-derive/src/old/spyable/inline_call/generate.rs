//! Code generation for the injected call recording.

use syn::parse_quote;

use crate::old::spyable::inline_call::info::SpyInlineCallInfo;

/// Inserts the call recording at the beginning of the original function block.
///
/// Unlike a fake, a spy never replaces the body: it records the call's arguments and then lets the
/// real implementation run.
///
/// Every argument is passed in a single tuple, and that tuple always carries a trailing comma, so
/// that a one-parameter function records a real 1-tuple (`&(id,)`) rather than a parenthesized
/// expression and a zero-parameter one records the unit type (`&()`). This mirrors how
/// `build_param_reference_tuple_type` builds the type the spy module expects.
pub fn insert_spy_inline_call_into_fn_block(
    original_block: &syn::Block,
    info: &SpyInlineCallInfo,
) -> syn::Block {
    let module_name = &info.module_name;
    let reference_call_values = &info.reference_call_values;

    parse_quote!({
        #[cfg(test)]
        self::#module_name::internal_record_call(&(#(#reference_call_values,)*));

        #original_block
    })
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_insert_spy_inline_call_into_fn_block_no_params() {
        let original_block: syn::Block = parse_quote!({ String::new() });
        let info = SpyInlineCallInfo {
            module_name: parse_quote!(get_user_spy_module),
            reference_call_values: vec![],
        };

        let block = insert_spy_inline_call_into_fn_block(&original_block, &info);

        let expected: syn::Block = parse_quote! {
            {
                #[cfg(test)]
                self::get_user_spy_module::internal_record_call(&());

                { String::new() }
            }
        };

        assert_eq!(
            block.to_token_stream().to_string(),
            expected.to_token_stream().to_string(),
            "the modified block should record a call with no arguments"
        );
    }

    #[test]
    fn test_insert_spy_inline_call_into_fn_block_single_param() {
        let original_block: syn::Block = parse_quote!({ id });
        let info = SpyInlineCallInfo {
            module_name: parse_quote!(get_user_spy_module),
            reference_call_values: vec![parse_quote!(id)],
        };

        let block = insert_spy_inline_call_into_fn_block(&original_block, &info);

        let expected: syn::Block = parse_quote! {
            {
                #[cfg(test)]
                self::get_user_spy_module::internal_record_call(&(id,));

                { id }
            }
        };

        assert_eq!(
            block.to_token_stream().to_string(),
            expected.to_token_stream().to_string(),
            "the modified block should record a call with a single argument"
        );
    }

    #[test]
    fn test_insert_spy_inline_call_into_fn_block_multiple_params() {
        let original_block: syn::Block = parse_quote!({ String::new() });
        let info = SpyInlineCallInfo {
            module_name: parse_quote!(update_user_spy_module),
            reference_call_values: vec![parse_quote!(id), parse_quote!(&name)],
        };

        let block = insert_spy_inline_call_into_fn_block(&original_block, &info);

        let expected: syn::Block = parse_quote! {
            {
                #[cfg(test)]
                self::update_user_spy_module::internal_record_call(&(id, &name,));

                { String::new() }
            }
        };

        assert_eq!(
            block.to_token_stream().to_string(),
            expected.to_token_stream().to_string(),
            "the modified block should record a call with multiple arguments"
        );
    }
}
