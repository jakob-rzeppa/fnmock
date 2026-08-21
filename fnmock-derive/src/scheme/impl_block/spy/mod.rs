use crate::{item_info::impl_block::ImplBlockInfo, scheme::impl_block::common::ImplCommonScheme};

pub struct ImplSpyScheme {
    pub common: ImplCommonScheme,
}

impl TryFrom<ImplBlockInfo> for ImplSpyScheme {
    type Error = syn::Error;

    fn try_from(_value: ImplBlockInfo) -> Result<Self, Self::Error> {
        todo!()
    }
}
