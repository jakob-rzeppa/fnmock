use crate::{
    item_info::impl_block::info::ImplBlockInfo, scheme::impl_block::common::ImplCommonScheme,
};

pub struct ImplFakeScheme {
    common: ImplCommonScheme,
}

impl TryFrom<ImplBlockInfo> for ImplFakeScheme {
    type Error = syn::Error;

    fn try_from(value: ImplBlockInfo) -> Result<Self, Self::Error> {
        todo!()
    }
}
