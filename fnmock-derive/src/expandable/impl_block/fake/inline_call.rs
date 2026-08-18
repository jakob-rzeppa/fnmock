use syn::parse_quote;

use crate::item_info::call_value::CallValue;

pub fn build_inline_call(
    module_name: &syn::Ident,
    fake_call_values: &[CallValue],
    generic_idents: Option<&[syn::Ident]>,
) -> syn::Block {
    if let Some(generic_idents) = generic_idents {
        parse_quote! {
            {
                if let Some(implementation) = self::#module_name::implementation::<#(#generic_idents),*>() {
                    return implementation(#(#fake_call_values),*);
                }
            }
        }
    } else {
        parse_quote! {
            {
                if let Some(implementation) = self::#module_name::implementation() {
                    return implementation(#(#fake_call_values),*);
                }
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
    fn test_no_generics_no_call_values() {
        let module_name: syn::Ident = parse_quote!(my_method_module);
        let fake_call_values: Vec<CallValue> = vec![];

        let res = build_inline_call(&module_name, &fake_call_values, None);

        let expected: syn::Block = parse_quote! {{
            if let Some(implementation) = self::my_method_module::implementation() {
                return implementation();
            }
        }};

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_no_generics_single_call_value() {
        let module_name: syn::Ident = parse_quote!(my_method_module);
        let fake_call_values: Vec<CallValue> = vec![CallValue::Ident(parse_quote!(a))];

        let res = build_inline_call(&module_name, &fake_call_values, None);

        let expected: syn::Block = parse_quote! {{
            if let Some(implementation) = self::my_method_module::implementation() {
                return implementation(a);
            }
        }};

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_no_generics_multiple_call_values() {
        let module_name: syn::Ident = parse_quote!(my_method_module);
        let fake_call_values: Vec<CallValue> = vec![
            CallValue::Ident(parse_quote!(a)),
            CallValue::Tuple(vec![
                CallValue::Ident(parse_quote!(b)),
                CallValue::Ident(parse_quote!(c)),
            ]),
            CallValue::Slice(vec![CallValue::Ident(parse_quote!(d))]),
        ];

        let res = build_inline_call(&module_name, &fake_call_values, None);

        let expected: syn::Block = parse_quote! {{
            if let Some(implementation) = self::my_method_module::implementation() {
                return implementation(a, (b, c), [d]);
            }
        }};

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_with_single_generic_no_call_values() {
        let module_name: syn::Ident = parse_quote!(my_method_module);
        let fake_call_values: Vec<CallValue> = vec![];
        let generic_idents: Vec<syn::Ident> = vec![parse_quote!(T)];

        let res = build_inline_call(&module_name, &fake_call_values, Some(&generic_idents));

        let expected: syn::Block = parse_quote! {{
            if let Some(implementation) = self::my_method_module::implementation::<T>() {
                return implementation();
            }
        }};

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_with_multiple_generics_and_call_values() {
        let module_name: syn::Ident = parse_quote!(my_method_module);
        let fake_call_values: Vec<CallValue> = vec![
            CallValue::Ident(parse_quote!(a)),
            CallValue::Ident(parse_quote!(b)),
        ];
        let generic_idents: Vec<syn::Ident> = vec![parse_quote!(T), parse_quote!(U)];

        let res = build_inline_call(&module_name, &fake_call_values, Some(&generic_idents));

        let expected: syn::Block = parse_quote! {{
            if let Some(implementation) = self::my_method_module::implementation::<T, U>() {
                return implementation(a, b);
            }
        }};

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_with_empty_generic_idents_slice_takes_generic_branch() {
        let module_name: syn::Ident = parse_quote!(my_method_module);
        let fake_call_values: Vec<CallValue> = vec![];
        let generic_idents: Vec<syn::Ident> = vec![];

        let res = build_inline_call(&module_name, &fake_call_values, Some(&generic_idents));

        let expected: syn::Block = parse_quote! {{
            if let Some(implementation) = self::my_method_module::implementation::<>() {
                return implementation();
            }
        }};

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_tuple_call_value_reconstructs_as_parenthesized_group() {
        let module_name: syn::Ident = parse_quote!(my_method_module);
        let fake_call_values: Vec<CallValue> = vec![CallValue::Tuple(vec![
            CallValue::Ident(parse_quote!(a)),
            CallValue::Ident(parse_quote!(b)),
        ])];

        let res = build_inline_call(&module_name, &fake_call_values, None);

        let expected: syn::Block = parse_quote! {{
            if let Some(implementation) = self::my_method_module::implementation() {
                return implementation((a, b));
            }
        }};

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_slice_call_value_reconstructs_as_bracketed_group() {
        let module_name: syn::Ident = parse_quote!(my_method_module);
        let fake_call_values: Vec<CallValue> = vec![CallValue::Slice(vec![
            CallValue::Ident(parse_quote!(a)),
            CallValue::Ident(parse_quote!(b)),
        ])];

        let res = build_inline_call(&module_name, &fake_call_values, None);

        let expected: syn::Block = parse_quote! {{
            if let Some(implementation) = self::my_method_module::implementation() {
                return implementation([a, b]);
            }
        }};

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_nested_tuple_and_slice_call_values_restructure_recursively() {
        let module_name: syn::Ident = parse_quote!(my_method_module);
        let fake_call_values: Vec<CallValue> = vec![CallValue::Tuple(vec![
            CallValue::Slice(vec![
                CallValue::Ident(parse_quote!(a)),
                CallValue::Ident(parse_quote!(b)),
            ]),
            CallValue::Tuple(vec![CallValue::Ident(parse_quote!(c))]),
        ])];

        let res = build_inline_call(&module_name, &fake_call_values, None);

        let expected: syn::Block = parse_quote! {{
            if let Some(implementation) = self::my_method_module::implementation() {
                return implementation(([a, b], (c)));
            }
        }};

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
}
