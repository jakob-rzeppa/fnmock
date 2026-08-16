use syn::parse_quote;

pub fn insert_inline_call(item_fn: &mut syn::ItemFn, inline_call: &syn::Block) {
    let original_block = &item_fn.block;

    let new_block: syn::Block = parse_quote!({
        #[cfg(test)]
        #inline_call

        #original_block
    });

    item_fn.block = Box::new(new_block);
}

#[cfg(test)]
mod tests {
    use quote::{ToTokens, quote};

    use super::*;

    #[test]
    fn test_insert_inline_call() {
        let mut item_fn: syn::ItemFn = parse_quote! {
            fn my_function() -> i32 {
                42
            }
        };
        let inline_call: syn::Block = parse_quote!({ my_inline_call(); });

        insert_inline_call(&mut item_fn, &inline_call);

        let expected: syn::ItemFn = parse_quote! {
            fn my_function() -> i32 {
                #[cfg(test)]
                {
                    my_inline_call();
                }

                {
                    42
                }
            }
        };

        assert_eq!(
            item_fn.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
}
