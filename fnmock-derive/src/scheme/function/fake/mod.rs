use crate::{
    item_info::function::info::FunctionInfo, scheme::function::common::FunctionCommonScheme,
};

pub struct FunctionFakeScheme {
    common: FunctionCommonScheme,
}

impl TryFrom<FunctionInfo> for FunctionFakeScheme {
    type Error = syn::Error;

    fn try_from(value: FunctionInfo) -> Result<Self, Self::Error> {
        todo!()
    }
}
