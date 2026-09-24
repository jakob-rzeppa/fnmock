use syn::parse_quote;

/// Concatenates the statements of two blocks, `first` before `second`.
pub fn merge_blocks(first: syn::Block, second: syn::Block) -> syn::Block {
    let first_stmts = first.stmts;
    let second_stmts = second.stmts;
    parse_quote! {
        {
            #(#first_stmts)*
            #(#second_stmts)*
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_statements_are_concatenated_in_order() {
        let first: syn::Block = parse_quote! { { record(&id); } };
        let second: syn::Block = parse_quote! { { if let Some(f) = get() { return f(id); } } };

        let merged = merge_blocks(first, second);

        let expected: syn::Block = parse_quote! {
            {
                record(&id);
                if let Some(f) = get() { return f(id); }
            }
        };
        assert_eq!(quote!(#merged).to_string(), quote!(#expected).to_string());
    }
}
