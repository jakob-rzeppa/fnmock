/// Generates documentation strings for fake proxy functions based on actual function parameters.

use quote::quote;

/// Builds documentation for fake proxy functions.
///
/// Generates parameter documentation, examples, and other descriptive text based on
/// the actual function signature.
pub(crate) struct FakeProxyDocs {
    param_docs: Vec<String>,
    return_type_str: String,
    setup_example: Vec<String>,
    is_async: bool,
}

impl FakeProxyDocs {
    /// Creates documentation for fake proxy functions.
    ///
    /// # Arguments
    ///
    /// * `fake_fn_name` - The name of the fake function/module
    /// * `fn_inputs` - The original function parameters
    /// * `return_type` - The return type of the function
    /// * `fn_asyncness` - Whether the function is async
    pub(crate) fn new(
        fake_fn_name: &syn::Ident,
        fn_inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
        return_type: &syn::Type,
        fn_asyncness: Option<syn::token::Async>,
    ) -> Self {
        let all_params: Vec<_> = fn_inputs
            .iter()
            .filter_map(|arg| {
                if let syn::FnArg::Typed(pat_type) = arg {
                    let name = &pat_type.pat;
                    let ty = &pat_type.ty;
                    Some((name, ty))
                } else {
                    None
                }
            })
            .collect();
        
        let param_docs: Vec<String> = all_params
            .iter()
            .map(|(name, ty)| {
                format!("* `{}: {}` - Parameter value", quote::quote!(#name), quote::quote!(#ty))
            })
            .collect();
        
        let return_type_str = quote::quote!(#return_type).to_string();
        
        let setup_example = if all_params.is_empty() {
            vec![
                format!("{}::setup(|| {{", fake_fn_name),
                "    // Full custom implementation".to_string(),
                format!("    {}", quote::quote!(#return_type)),
                "});".to_string(),
            ]
        } else {
            let example_params: Vec<_> = all_params
                .iter()
                .map(|(name, _)| quote::quote!(#name))
                .collect();
            
            let params_pattern = if example_params.len() == 1 {
                quote::quote!(#(#example_params)*)
            } else {
                quote::quote!((#(#example_params),*))
            };
            
            vec![
                format!("{}::setup(|{}| {{", fake_fn_name, quote::quote!(#params_pattern)),
                "    // Full custom implementation".to_string(),
                "    if condition {".to_string(),
                format!("        {}", quote::quote!(#return_type)),
                "    } else {".to_string(),
                format!("        {}", quote::quote!(#return_type)),
                "    }".to_string(),
                "});".to_string(),
            ]
        };

        Self {
            param_docs,
            return_type_str,
            setup_example,
            is_async: fn_asyncness.is_some(),
        }
    }

    /// Generates documentation attributes for the `setup` function.
    pub(crate) fn setup_docs(&self) -> proc_macro2::TokenStream {
        let return_type_str = &self.return_type_str;
        
        let mut docs = vec![
            quote! { #[doc = "Sets up the fake's implementation."] },
            quote! { #[doc = ""] },
            quote! { #[doc = "Configures the function that will be executed when the fake is called."] },
            quote! { #[doc = "Unlike mocks (which track calls) or stubs (which return fixed values),"] },
            quote! { #[doc = "fakes provide full custom implementations that can contain complex logic."] },
        ];
        
        if self.is_async {
            docs.extend(vec![
                quote! { #[doc = ""] },
                quote! { #[doc = "# Note"] },
                quote! { #[doc = ""] },
                quote! { #[doc = "This function is async, but the fake implementation function must be sync."] },
                quote! { #[doc = "The fake will automatically wrap the return value."] },
            ]);
        }
        
        docs.extend(vec![
            quote! { #[doc = ""] },
            quote! { #[doc = "# Parameters"] },
            quote! { #[doc = ""] },
        ]);
        
        if self.param_docs.is_empty() {
            docs.push(quote! { #[doc = "No parameters"] });
        } else {
            for param in &self.param_docs {
                docs.push(quote! { #[doc = #param] });
            }
        }
        
        docs.extend(vec![
            quote! { #[doc = ""] },
            quote! { #[doc = "# Returns"] },
            quote! { #[doc = ""] },
            quote! { #[doc = #return_type_str] },
            quote! { #[doc = ""] },
            quote! { #[doc = "# Examples"] },
            quote! { #[doc = ""] },
            quote! { #[doc = "```ignore"] },
        ]);
        
        for line in &self.setup_example {
            docs.push(quote! { #[doc = #line] });
        }
        
        docs.push(quote! { #[doc = "```"] });
        
        quote! { #(#docs)* }
    }

    /// Generates documentation attributes for the `clear` function.
    pub(crate) fn clear_docs(&self) -> proc_macro2::TokenStream {
        quote! {
            #[doc = "Clears the fake state."]
            #[doc = ""]
            #[doc = "Resets the fake by clearing the configured implementation."]
            #[doc = "After calling `clear()`, the fake will panic if invoked before"]
            #[doc = "calling `setup()` again."]
        }
    }

    /// Generates documentation attributes for the `get_implementation` function.
    pub(crate) fn get_implementation_docs(&self) -> proc_macro2::TokenStream {
        quote! {
            #[doc = "Gets the configured implementation."]
            #[doc = ""]
            #[doc = "This function is used internally by the fake function to retrieve"]
            #[doc = "the implementation that was configured via `setup()`."]
            #[doc = ""]
            #[doc = "# Returns"]
            #[doc = ""]
            #[doc = "The configured function implementation"]
            #[doc = ""]
            #[doc = "# Panics"]
            #[doc = ""]
            #[doc = "Panics if `setup()` has not been called before calling the fake function"]
        }
    }
}
