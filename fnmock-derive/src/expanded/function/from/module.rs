use syn::parse_quote;

pub fn build_module(
    vis: &syn::Visibility,
    name: &syn::Ident,
    parts: &[proc_macro2::TokenStream],
) -> syn::ItemMod {
    parse_quote! {
        #[cfg(test)]
        #vis mod #name {
            use super::*;

            #(#parts)*
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::{ToTokens, quote};

    use super::*;

    #[test]
    fn test_build_module() {
        let vis: syn::Visibility = parse_quote!(pub);
        let name: syn::Ident = parse_quote!(my_function_module);
        let parts: Vec<proc_macro2::TokenStream> = vec![
            quote! {
                pub fn get_fake_interface() -> FakeInterface {
                    FakeInterface {}
                }
            },
            quote! {
                pub struct FakeInterface {}
            },
        ];

        let module = build_module(&vis, &name, &parts);

        let expected: syn::ItemMod = parse_quote! {
            #[cfg(test)]
            pub mod my_function_module {
                use super::*;

                pub fn get_fake_interface() -> FakeInterface {
                    FakeInterface {}
                }

                pub struct FakeInterface {}
            }
        };

        assert_eq!(
            module.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
}
