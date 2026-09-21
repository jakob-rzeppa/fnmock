use quote::quote;

use crate::expanded::impl_block::ImplExpanded;

impl From<ImplExpanded> for proc_macro2::TokenStream {
    fn from(val: ImplExpanded) -> Self {
        let ImplExpanded {
            impl_with_inline_calls,
            accessor_impl_block,
            modules,
        } = val;

        quote! {
            #impl_with_inline_calls

            #accessor_impl_block

            #(#modules)*
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_impl_expanded_into_token_stream() {
        let impl_expanded = ImplExpanded {
            impl_with_inline_calls: parse_quote!(
                impl MyStruct {
                    fn my_method(&self) -> i32 {
                        #[cfg(test)]
                        {
                            my_inline_call();
                        }
                        42
                    }

                    fn my_other_method(&self) -> String {
                        #[cfg(test)]
                        {
                            my_other_inline_call();
                        }
                        "Hello, world!".to_string()
                    }
                }
            ),
            accessor_impl_block: parse_quote!(
                impl MyStruct {
                    fn get_value(&self) -> i32 {
                        self.value
                    }
                }
            ),
            modules: vec![
                parse_quote!(
                    #[cfg(test)]
                    mod my_module {
                        // My module
                    }
                ),
                parse_quote!(
                    #[cfg(test)]
                    mod my_other_module {
                        // My other module
                    }
                ),
            ],
        };

        let token_stream: proc_macro2::TokenStream = impl_expanded.into();

        let expected_token_stream = quote!(
            impl MyStruct {
                fn my_method(&self) -> i32 {
                    #[cfg(test)]
                    {
                        my_inline_call();
                    }
                    42
                }

                fn my_other_method(&self) -> String {
                    #[cfg(test)]
                    {
                        my_other_inline_call();
                    }
                    "Hello, world!".to_string()
                }
            }

            impl MyStruct {
                fn get_value(&self) -> i32 {
                    self.value
                }
            }

            #[cfg(test)]
            mod my_module {
                // My module
            }

            #[cfg(test)]
            mod my_other_module {
                // My other module
            }
        );

        assert_eq!(token_stream.to_string(), expected_token_stream.to_string());
    }
}
