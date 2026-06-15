use quote::{ format_ident, quote };
use syn::parse_macro_input;

use crate::helpers::{ pascal_to_snake_case, snake_to_pascal_case };

enum FakeInput {
    Function {
        ident: syn::Ident,
        generics: Option<syn::AngleBracketedGenericArguments>,
    },
    StructMethod {
        struct_path: syn::TypePath,
        struct_generics: Option<syn::AngleBracketedGenericArguments>,
        method_ident: syn::Ident,
        method_generics: Option<syn::AngleBracketedGenericArguments>,
    },
}

impl syn::parse::Parse for FakeInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Collect the tokens for the function or struct path until we encounter a `<` or `,`
        let mut function_ident_or_struct_path = proc_macro2::TokenStream::new();
        while !input.is_empty() && !input.peek(syn::Token![<]) && !input.peek(syn::Token![,]) {
            let tt: proc_macro2::TokenTree = input.parse()?;
            function_ident_or_struct_path.extend(std::iter::once(tt));
        }

        // Check if we have a `<` indicating generics
        let function_or_struct_generics = if input.peek(syn::Token![<]) {
            Some(input.parse::<syn::AngleBracketedGenericArguments>()?)
        } else {
            None
        };

        // Check if we have a `,` indicating a struct method
        if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?; // Consume the comma
        } else {
            // If there's no comma, we assume it's a function
            let ident = syn::parse2(function_ident_or_struct_path)?;
            return Ok(FakeInput::Function {
                ident,
                generics: function_or_struct_generics,
            });
        }

        // Now that we know it's a struct method, we need to parse the struct path and method identifier
        let struct_path: syn::TypePath = syn::parse2(function_ident_or_struct_path)?;
        let struct_generics = function_or_struct_generics;

        // If we have a comma, we expect a method identifier and optional generics
        let method_ident: syn::Ident = input.parse()?;
        let method_generics = if input.peek(syn::Token![<]) {
            Some(input.parse::<syn::AngleBracketedGenericArguments>()?)
        } else {
            None
        };

        Ok(FakeInput::StructMethod {
            struct_path,
            struct_generics,
            method_ident,
            method_generics,
        })
    }
}

pub fn handle_fake(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as FakeInput);

    let expanded = match input {
        FakeInput::Function { ident, generics } => {
            let module_name = format_ident!("{}_fake", ident);
            let interface_struct_name = format_ident!(
                "{}FakeInterface",
                snake_to_pascal_case(&ident.to_string())
            );

            if let Some(generics) = generics {
                quote! {
                    #module_name::#interface_struct_name::#generics::new()
                }
            } else {
                quote! {
                    #module_name::#interface_struct_name::new()
                }
            }
        }
        FakeInput::StructMethod { struct_path, struct_generics, method_ident, method_generics } => {
            let struct_name = struct_path.path.segments
                .last()
                .expect("Struct name could not be determined from path.")
                .ident.to_string();

            let module_name = format_ident!(
                "{}_{}_fake",
                pascal_to_snake_case(&struct_name),
                method_ident
            );
            let interface_struct_name = format_ident!(
                "{}{}FakeInterface",
                struct_name.to_string(),
                snake_to_pascal_case(&method_ident.to_string())
            );

            let combined_generics = match (struct_generics, method_generics) {
                (Some(struct_gens), Some(method_gens)) => {
                    let mut combined = struct_gens.clone();
                    combined.args.extend(method_gens.args);
                    Some(combined)
                }
                (Some(struct_gens), None) => Some(struct_gens),
                (None, Some(method_gens)) => Some(method_gens),
                (None, None) => None,
            };

            if let Some(generics) = combined_generics {
                quote! {
                    #module_name::#interface_struct_name::#generics::new()
                }
            } else {
                quote! {
                    #module_name::#interface_struct_name::new()
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
