use crate::{
    item_info::impl_block::info::ImplBlockInfo, scheme::impl_block::common::ImplCommonScheme,
};

pub struct ImplSpyScheme {
    common: ImplCommonScheme,
}

impl TryFrom<ImplBlockInfo> for ImplSpyScheme {
    type Error = syn::Error;

    fn try_from(value: ImplBlockInfo) -> Result<Self, Self::Error> {
        todo!()
    }
}
