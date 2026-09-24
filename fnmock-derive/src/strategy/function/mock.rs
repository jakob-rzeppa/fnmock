use crate::{
    expandable::function::FunctionExpandable, expanded::function::FunctionExpanded,
    item_info::function::FunctionInfo, scheme::mock::function::FunctionMockScheme,
    strategy::Strategy,
};

pub struct FunctionMockStrategy;

impl Strategy for FunctionMockStrategy {
    type Item = syn::ItemFn;
    type ItemInfo = FunctionInfo;
    type Scheme = FunctionMockScheme;
    type Expandable = FunctionExpandable;
    type Expanded = FunctionExpanded;
}
