use syn::parse_quote;

/// Builds the block injected into the spied function's body: a call to `internal_record_call`
/// passing one argument per parameter (not a single params value — see
/// [`build_record_call`](super::module::record_call::build_record_call)'s doc comment for why).
pub fn build_inline_call(
    module_name: &syn::Ident,
    reference_call_values: &[syn::Expr],
    generic_idents: Option<&[syn::Ident]>,
) -> syn::Block {
    if let Some(generic_idents) = generic_idents {
        parse_quote! {
            {
                self::#module_name::internal_record_call::<#(#generic_idents),*>(#(#reference_call_values),*);
            }
        }
    } else {
        parse_quote! {
            {
                self::#module_name::internal_record_call(#(#reference_call_values),*);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_no_generics_no_params() {
        let module_name: syn::Ident = parse_quote!(my_function_spy_module);
        let values: Vec<syn::Expr> = vec![];

        let res = build_inline_call(&module_name, &values, None);

        let expected: syn::Block = parse_quote! {{
            self::my_function_spy_module::internal_record_call();
        }};

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_no_generics_single_param() {
        let module_name: syn::Ident = parse_quote!(my_function_spy_module);
        let values: Vec<syn::Expr> = vec![parse_quote!(&id)];

        let res = build_inline_call(&module_name, &values, None);

        let expected: syn::Block = parse_quote! {{
            self::my_function_spy_module::internal_record_call(&id);
        }};

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_no_generics_multiple_params() {
        let module_name: syn::Ident = parse_quote!(my_function_spy_module);
        let values: Vec<syn::Expr> = vec![parse_quote!(&id), parse_quote!(uuid)];

        let res = build_inline_call(&module_name, &values, None);

        let expected: syn::Block = parse_quote! {{
            self::my_function_spy_module::internal_record_call(&id, uuid);
        }};

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_with_generics() {
        let module_name: syn::Ident = parse_quote!(my_function_spy_module);
        let values: Vec<syn::Expr> = vec![parse_quote!(&a)];
        let generic_idents: Vec<syn::Ident> = vec![parse_quote!(T)];

        let res = build_inline_call(&module_name, &values, Some(&generic_idents));

        let expected: syn::Block = parse_quote! {{
            self::my_function_spy_module::internal_record_call::<T>(&a);
        }};

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
}
