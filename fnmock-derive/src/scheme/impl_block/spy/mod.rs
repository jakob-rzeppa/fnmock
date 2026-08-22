use crate::{item_info::impl_block::ImplBlockInfo, scheme::impl_block::common::ImplCommonScheme};

pub struct ImplSpyScheme {
    pub common: ImplCommonScheme,
}

impl TryFrom<ImplBlockInfo> for ImplSpyScheme {
    type Error = syn::Error;

    fn try_from(value: ImplBlockInfo) -> Result<Self, Self::Error> {
        Err(syn::Error::new_spanned(
            &value.struct_name,
            "The #[spyable] attribute does not support impl blocks yet. Only free functions can be spied on.",
        ))
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_impl_block_is_rejected_with_a_message_naming_impl_blocks() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl UserService {
                fn get_user(&self, id: String) -> String {
                    id
                }
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid inherent impl block");

        let result = ImplSpyScheme::try_from(info);

        let Err(error) = result else {
            panic!("impl blocks are not supported yet and should be rejected");
        };
        assert!(
            error.to_string().contains("impl blocks"),
            "the error should say impl blocks are unsupported, got: {error}"
        );
    }
}
