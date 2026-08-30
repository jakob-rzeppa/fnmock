use quote::quote;

use crate::expanded::function::FunctionExpanded;

impl From<FunctionExpanded> for proc_macro2::TokenStream {
    fn from(val: FunctionExpanded) -> Self {
        let FunctionExpanded {
            fn_with_inline_call,
            accessor_fn,
            module,
        } = val;

        quote! {
            #fn_with_inline_call

            #accessor_fn

            #module
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_function_expanded_into_token_stream() {
        let function_expanded = FunctionExpanded {
            fn_with_inline_call: parse_quote!(
                fn my_function() -> i32 {
                    #[cfg(test)]
                    {
                        my_inline_call();
                    }
                    42
                }
            ),
            accessor_fn: parse_quote!(
                fn my_function_fake() -> FakeInterface {
                    my_function_module::get_fake_interface()
                }
            ),
            module: parse_quote!(
                #[cfg(test)]
                mod my_function_module {
                    use super::*;

                    pub fn get_fake_interface() -> FakeInterface {
                        // Return a fake interface for testing
                        FakeInterface {}
                    }

                    pub struct FakeInterface {}
                }
            ),
        };

        let token_stream: proc_macro2::TokenStream = function_expanded.into();

        let expected_token_stream = quote! {
            fn my_function() -> i32 {
                #[cfg(test)]
                {
                    my_inline_call();
                }
                42
            }

            fn my_function_fake() -> FakeInterface {
                my_function_module::get_fake_interface()
            }

            #[cfg(test)]
            mod my_function_module {
                use super::*;

                pub fn get_fake_interface() -> FakeInterface {
                    // Return a fake interface for testing
                    FakeInterface {}
                }

                pub struct FakeInterface {}
            }
        };

        assert_eq!(token_stream.to_string(), expected_token_stream.to_string());
    }
}
