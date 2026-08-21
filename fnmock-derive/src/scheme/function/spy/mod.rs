use crate::{item_info::function::FunctionInfo, scheme::function::common::FunctionCommonScheme};

pub struct FunctionSpyScheme {
    pub common: FunctionCommonScheme,
}

impl TryFrom<FunctionInfo> for FunctionSpyScheme {
    type Error = syn::Error;

    fn try_from(_value: FunctionInfo) -> Result<Self, Self::Error> {
        todo!()
    }
}
