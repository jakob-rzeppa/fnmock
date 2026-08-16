use crate::{item_info::function::info::FunctionInfo, plan::function::common::FunctionCommonPlan};

pub struct FunctionSpyPlan {
    common: FunctionCommonPlan,
}

impl TryFrom<FunctionInfo> for FunctionSpyPlan {
    type Error = syn::Error;

    fn try_from(value: FunctionInfo) -> Result<Self, Self::Error> {
        todo!()
    }
}
