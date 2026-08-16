use crate::{item_info::impl_block::info::ImplBlockInfo, plan::impl_block::common::ImplCommonPlan};

pub struct ImplFakePlan {
    common: ImplCommonPlan,
}

impl TryFrom<ImplBlockInfo> for ImplFakePlan {
    type Error = syn::Error;

    fn try_from(value: ImplBlockInfo) -> Result<Self, Self::Error> {
        todo!()
    }
}
