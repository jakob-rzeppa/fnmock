//! Code generation for the injected fake lookup.

use quote::quote;

use crate::fakeable::inline_call::info::FakeInlineCallInfo;

/// Inserts an inline call to the fake implementation at the beginning of the original function block.
///
/// If the fake is set, it will call the fake implementation and return its result. Otherwise, it will execute the original function block.
///
/// # Errors
///
/// Returns an error if the generated block fails to parse, which would be a bug in fnmock.
pub fn insert_inline_call_into_fn_block(
    original_block: &syn::Block,
    inline_call_info: &FakeInlineCallInfo,
) -> syn::Result<syn::Block> {
    let fake_call_values = &inline_call_info.fake_call_values;
    let module_name = &inline_call_info.module_name;
    let interface_struct_name = &inline_call_info.interface_struct_name;

    let new_block = if let Some(generic_idents) = &inline_call_info.generic_idents {
        quote! {
            {
                #[cfg(test)]
                {
                    let fake = self::#module_name::#interface_struct_name::<#(#generic_idents),*>::new();
                    if fake.is_set() {
                        let implementation = fake.get();
                        return implementation(#(#fake_call_values),*);
                    }
                }

                #original_block
            }
        }
    } else {
        quote! {
            {
                #[cfg(test)]
                {
                    let fake = self::#module_name::#interface_struct_name::new();
                    if fake.is_set() {
                        let implementation = fake.get();
                        return implementation(#(#fake_call_values),*);
                    }
                }

                #original_block
            }
        }
    };

    syn::parse2(new_block).map_err(|e|
        syn::Error::new_spanned(
            original_block,
            format!(
                "internal error: failed to parse the function block after inserting the fake lookup: {e}. This is a bug in fnmock; please report it."
            )
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::function::extract_function_info;
    use crate::fakeable::inline_call::info::FakeInlineCallInfo;
    use crate::names::NameType;
    use quote::ToTokens;
    use syn::parse_quote;

    #[test]
    fn test_insert_inline_call_into_fn_block_non_generic_function() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            fn get_user(id: u32) -> String {
                String::new()
            }
        };
        let function_info =
            extract_function_info(&item_fn, NameType::Fake).expect("valid standalone function");
        let inline_call_info = FakeInlineCallInfo::try_from(&function_info)
            .expect("conversion should succeed for a non-generic standalone function");

        let block = insert_inline_call_into_fn_block(&item_fn.block, &inline_call_info)
            .expect("inserting the inline call should succeed for a non-generic function");

        let expected: syn::Block = parse_quote! {
            {
                #[cfg(test)]
                {
                    let fake = self::get_user_fake_module::GetUserFakeInterface::new();
                    if fake.is_set() {
                        let implementation = fake.get();
                        return implementation(id);
                    }
                }

                { String::new() }
            }
        };

        assert_eq!(
            block.to_token_stream().to_string(),
            expected.to_token_stream().to_string(),
            "the modified block should match the expected structure"
        );
    }

    #[test]
    fn test_insert_inline_call_into_fn_block_generic_function() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            fn compute<T>(x: T) -> T {
                x
            }
        };
        let function_info = extract_function_info(&item_fn, NameType::Fake)
            .expect("valid generic standalone function");
        let inline_call_info = FakeInlineCallInfo::try_from(&function_info)
            .expect("conversion should succeed for a generic standalone function");

        let block = insert_inline_call_into_fn_block(&item_fn.block, &inline_call_info)
            .expect("inserting the inline call should succeed for a generic function");

        let expected: syn::Block = parse_quote! {
            {
                #[cfg(test)]
                {
                    let fake = self::compute_fake_module::ComputeFakeInterface::<T>::new();
                    if fake.is_set() {
                        let implementation = fake.get();
                        return implementation(x);
                    }
                }

                { x }
            }
        };

        assert_eq!(
            block.to_token_stream().to_string(),
            expected.to_token_stream().to_string(),
            "the modified block should match the expected structure for a generic function"
        );
    }
}
