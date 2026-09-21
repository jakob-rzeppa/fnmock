use syn::parse_quote;

/// A free function's original body, before fnmock injects its inline call.
pub struct OriginalFn(syn::ItemFn);

impl OriginalFn {
    pub fn new(item_fn: syn::ItemFn) -> Self {
        Self(item_fn)
    }

    /// Consumes the original function and returns it with `inline_call` spliced in under
    /// `#[cfg(test)]`, ahead of the original body. The only mutation the pipeline performs on the
    /// user's function, and the only way to get a `syn::ItemFn` back out of an [`OriginalFn`].
    pub fn into_fn_with_inline_call(self, inline_call: &syn::Block) -> syn::ItemFn {
        let mut item_fn = self.0;
        let original_block = &item_fn.block;

        let new_block: syn::Block = parse_quote!({
            #[cfg(test)]
            #inline_call

            #original_block
        });

        item_fn.block = Box::new(new_block);
        item_fn
    }
}

/// An inherent impl block's original items, before fnmock injects each method's inline call.
pub struct OriginalImpl(syn::ItemImpl);

impl OriginalImpl {
    pub fn new(item_impl: syn::ItemImpl) -> Self {
        Self(item_impl)
    }

    /// Consumes the original impl block and returns it with each named method's inline call
    /// spliced in under `#[cfg(test)]`, ahead of that method's original body.
    ///
    /// `inline_calls` is `(method_name, inline_call)` pairs;
    /// every name must match a method in the impl block.
    pub fn into_impl_with_inline_calls(
        self,
        inline_calls: &[(syn::Ident, syn::Block)],
    ) -> syn::Result<syn::ItemImpl> {
        let mut item_impl = self.0;

        for (method_name, inline_call) in inline_calls {
            let method: &mut syn::ImplItemFn = item_impl
                .items
                .iter_mut()
                .find_map(|item| {
                    if let syn::ImplItem::Fn(method) = item {
                        if method.sig.ident == *method_name {
                            return Some(method);
                        }
                    }
                    None
                })
                .ok_or_else(|| {
                    syn::Error::new_spanned(
                        method_name,
                        format!(
                            "Method {method_name} not found in impl block. This is an error in fnmock. Please report this bug."
                        ),
                    )
                })?;

            let original_block = &method.block;
            let new_block: syn::Block = parse_quote!({
                #[cfg(test)]
                #inline_call

                #original_block
            });

            method.block = new_block;
        }

        Ok(item_impl)
    }
}

impl quote::ToTokens for OriginalImpl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_original_fn_into_fn_with_inline_call() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn my_function() -> i32 {
                42
            }
        };
        let inline_call: syn::Block = parse_quote!({
            my_inline_call();
        });

        let res = OriginalFn::new(item_fn).into_fn_with_inline_call(&inline_call);

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
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_original_impl_into_impl_with_inline_calls_single_method() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                fn my_method(&self) -> i32 {
                    42
                }
            }
        };

        let inline_calls = vec![(
            parse_quote!(my_method),
            parse_quote!({
                my_inline_call();
            }),
        )];

        let res = OriginalImpl::new(item_impl)
            .into_impl_with_inline_calls(&inline_calls)
            .unwrap();

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
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_original_impl_into_impl_with_inline_calls_only_targets_named_methods() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                fn method_a(&self) -> i32 {
                    1
                }

                fn method_b(&self) -> i32 {
                    2
                }
            }
        };

        let inline_calls = vec![(
            parse_quote!(method_a),
            parse_quote!({
                inline_call_a();
            }),
        )];

        let res = OriginalImpl::new(item_impl)
            .into_impl_with_inline_calls(&inline_calls)
            .unwrap();

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
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_original_impl_into_impl_with_inline_calls_empty_leaves_impl_unchanged() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                fn my_method(&self) -> i32 {
                    42
                }
            }
        };

        let expected = item_impl.clone();

        let res = OriginalImpl::new(item_impl)
            .into_impl_with_inline_calls(&[])
            .unwrap();

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_original_impl_into_impl_with_inline_calls_method_not_found_is_a_spanned_error() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                fn my_method(&self) -> i32 {
                    42
                }
            }
        };

        let inline_calls = vec![(
            parse_quote!(non_existent_method),
            parse_quote!({
                inline_call();
            }),
        )];

        let result = OriginalImpl::new(item_impl).into_impl_with_inline_calls(&inline_calls);

        let err = match result {
            Err(err) => err,
            Ok(_) => {
                panic!("expected a method missing from the impl block to be a syn::Error, not Ok")
            }
        };
        assert!(
            err.to_string().contains(
                "Method non_existent_method not found in impl block. This is an error in fnmock. Please report this bug."
            )
        );
    }

    #[test]
    fn test_original_impl_into_impl_with_inline_calls_multiple_methods() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl MyStruct {
                fn method_a(&self) -> i32 {
                    1
                }

                fn method_b(&self) -> i32 {
                    2
                }
            }
        };

        let inline_calls = vec![
            (
                parse_quote!(method_a),
                parse_quote!({
                    inline_call_a();
                }),
            ),
            (
                parse_quote!(method_b),
                parse_quote!({
                    inline_call_b();
                }),
            ),
        ];

        let res = OriginalImpl::new(item_impl)
            .into_impl_with_inline_calls(&inline_calls)
            .unwrap();

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
                    #[cfg(test)]
                    {
                        inline_call_b();
                    }

                    {
                        2
                    }
                }
            }
        };

        assert_eq!(
            res.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
}
