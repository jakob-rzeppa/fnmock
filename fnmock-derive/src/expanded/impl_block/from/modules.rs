use syn::parse_quote;

pub struct ModuleInfo<'a> {
    pub vis: &'a syn::Visibility,
    pub name: &'a syn::Ident,
    pub parts: &'a [proc_macro2::TokenStream],
}

pub fn build_modules(info: &[ModuleInfo<'_>]) -> Vec<syn::ItemMod> {
    info.iter()
        .map(|module_info| build_module(module_info.vis, module_info.name, module_info.parts))
        .collect()
}

fn build_module(
    vis: &syn::Visibility,
    name: &syn::Ident,
    parts: &[proc_macro2::TokenStream],
) -> syn::ItemMod {
    parse_quote! {
        #[cfg(test)]
        #vis mod #name {
            #(#parts)*
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::{ToTokens, quote};

    use super::*;

    #[test]
    fn test_build_modules_empty() {
        let info: Vec<ModuleInfo<'_>> = vec![];

        let res = build_modules(&info);

        assert!(res.is_empty());
    }

    #[test]
    fn test_build_modules_single() {
        let vis: syn::Visibility = parse_quote!(pub);
        let name: syn::Ident = parse_quote!(my_method_module);
        let parts: Vec<proc_macro2::TokenStream> = vec![quote! {
            pub fn get_fake_interface() -> FakeInterface {
                FakeInterface {}
            }
        }];

        let info = vec![ModuleInfo {
            vis: &vis,
            name: &name,
            parts: &parts,
        }];

        let res = build_modules(&info);

        let expected: syn::ItemMod = parse_quote! {
            #[cfg(test)]
            pub mod my_method_module {
                pub fn get_fake_interface() -> FakeInterface {
                    FakeInterface {}
                }
            }
        };

        assert_eq!(res.len(), 1);
        assert_eq!(
            res[0].to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_build_modules_multiple() {
        let vis_a: syn::Visibility = parse_quote!(pub);
        let name_a: syn::Ident = parse_quote!(method_a_module);
        let parts_a: Vec<proc_macro2::TokenStream> = vec![];

        let vis_b: syn::Visibility = syn::Visibility::Inherited;
        let name_b: syn::Ident = parse_quote!(method_b_module);
        let parts_b: Vec<proc_macro2::TokenStream> = vec![];

        let info = vec![
            ModuleInfo {
                vis: &vis_a,
                name: &name_a,
                parts: &parts_a,
            },
            ModuleInfo {
                vis: &vis_b,
                name: &name_b,
                parts: &parts_b,
            },
        ];

        let res = build_modules(&info);

        let expected_a: syn::ItemMod = parse_quote! {
            #[cfg(test)]
            pub mod method_a_module {}
        };
        let expected_b: syn::ItemMod = parse_quote! {
            #[cfg(test)]
            mod method_b_module {}
        };

        assert_eq!(res.len(), 2);
        assert_eq!(
            res[0].to_token_stream().to_string(),
            expected_a.to_token_stream().to_string()
        );
        assert_eq!(
            res[1].to_token_stream().to_string(),
            expected_b.to_token_stream().to_string()
        );
    }
}
