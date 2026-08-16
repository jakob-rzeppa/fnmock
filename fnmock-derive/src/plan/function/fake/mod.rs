use crate::{item_info::function::info::FunctionInfo, plan::function::common::FunctionCommonPlan};

pub struct FunctionFakePlan {
    common: FunctionCommonPlan,
}

impl TryFrom<FunctionInfo> for FunctionFakePlan {
    type Error = syn::Error;

    fn try_from(value: FunctionInfo) -> Result<Self, Self::Error> {
        todo!()
    }
}
