use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Token;

/// Structure to parse the mock_function attribute arguments
pub(crate) struct MockFunctionArgs {
    pub(crate) ignore: Vec<String>,
}

impl Parse for MockFunctionArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ignore = Vec::new();

        if input.is_empty() {
            return Ok(MockFunctionArgs { ignore });
        }

        // Parse "ignore = [...]" syntax
        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            if key == "ignore" {
                input.parse::<Token![=]>()?;
                let content;
                syn::bracketed!(content in input);
                let names: Punctuated<syn::Ident, Token![,]> = content.parse_terminated(syn::Ident::parse, Token![,])?;
                ignore = names.into_iter().map(|id| id.to_string()).collect();
            }

            // Allow trailing comma or end of input
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(MockFunctionArgs { ignore })
    }
}