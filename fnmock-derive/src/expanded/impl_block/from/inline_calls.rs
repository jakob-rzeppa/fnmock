use std::collections::HashMap;

use syn::parse_quote;

pub fn insert_inline_calls(
    item_impl: &mut syn::ItemImpl,
    // A HashMap where the key is the method name and the value is the inline call to be inserted.
    inline_calls: &HashMap<syn::Ident, syn::Block>,
) {
    for (method_name, inline_call) in inline_calls {
        let item: &mut syn::ImplItemFn = item_impl
            .items
            .iter_mut()
            .find_map(|item| {
                if let syn::ImplItem::Fn(method) = item {
                    if method.sig.ident == *method_name {
                        return Some(method);
                    }
                }
                None
            }).unwrap_or_else(|| unreachable!("Method {} not found in impl block. This is a error in fnmock. Please report this bug.", method_name));

        insert_inline_call(item, inline_call);
    }
}

fn insert_inline_call(impl_item_fn: &mut syn::ImplItemFn, inline_call: &syn::Block) {
    let original_block = &impl_item_fn.block;

    let new_block: syn::Block = parse_quote!({
        #[cfg(test)]
        #inline_call

        #original_block
    });

    impl_item_fn.block = new_block;
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    #[test]
    fn test_insert_inline_calls_single_method() {
        let mut item_impl: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                fn my_method(&self) -> i32 {
                    42
                }
            }
        };

        let mut inline_calls = HashMap::new();
        inline_calls.insert(
            parse_quote!(my_method),
            parse_quote!({
                my_inline_call();
            }),
        );

        insert_inline_calls(&mut item_impl, &inline_calls);

        let expected: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                fn my_method(&self) -> i32 {
                    #[cfg(test)]
                    {
                        my_inline_call();
                    }

                    {
                        42
                    }
                }
            }
        };

        assert_eq!(
            item_impl.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_insert_inline_calls_only_targets_named_methods() {
        let mut item_impl: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                fn method_a(&self) -> i32 {
                    1
                }

                fn method_b(&self) -> i32 {
                    2
                }
            }
        };

        let mut inline_calls = HashMap::new();
        inline_calls.insert(
            parse_quote!(method_a),
            parse_quote!({
                inline_call_a();
            }),
        );

        insert_inline_calls(&mut item_impl, &inline_calls);

        let expected: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                fn method_a(&self) -> i32 {
                    #[cfg(test)]
                    {
                        inline_call_a();
                    }

                    {
                        1
                    }
                }

                fn method_b(&self) -> i32 {
                    2
                }
            }
        };

        assert_eq!(
            item_impl.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_insert_inline_calls_empty_map_leaves_impl_unchanged() {
        let mut item_impl: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                fn my_method(&self) -> i32 {
                    42
                }
            }
        };

        let inline_calls = HashMap::new();

        insert_inline_calls(&mut item_impl, &inline_calls);

        let expected: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                fn my_method(&self) -> i32 {
                    42
                }
            }
        };

        assert_eq!(
            item_impl.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_insert_inline_calls_method_not_found_panics() {
        let mut item_impl: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                fn my_method(&self) -> i32 {
                    42
                }
            }
        };

        let mut inline_calls = HashMap::new();
        inline_calls.insert(
            parse_quote!(non_existent_method),
            parse_quote!({
                inline_call();
            }),
        );

        let result = std::panic::catch_unwind(move || {
            insert_inline_calls(&mut item_impl, &inline_calls);
        });

        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .downcast_ref::<String>()
            .unwrap()
            .contains("Method non_existent_method not found in impl block. This is a error in fnmock. Please report this bug."));
    }
}
