use crate::{item_info::impl_block::info::ImplBlockInfo, plan::impl_block::common::ImplCommonPlan};

pub struct ImplSpyPlan {
    common: ImplCommonPlan,
}

impl TryFrom<ImplBlockInfo> for ImplSpyPlan {
    type Error = syn::Error;

    fn try_from(value: ImplBlockInfo) -> Result<Self, Self::Error> {
        todo!()
    }
}
