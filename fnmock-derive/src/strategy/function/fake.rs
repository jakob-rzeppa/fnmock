use crate::{
    expandable::function::FunctionExpandable, expanded::function::FunctionExpanded,
    item_info::function::info::FunctionInfo, plan::function::fake::FunctionFakePlan,
    strategy::Strategy,
};

pub struct FunctionFakeStrategy;

impl Strategy for FunctionFakeStrategy {
    type Item = syn::ItemFn;
    type ItemInfo = FunctionInfo;
    type Plan = FunctionFakePlan;
    type Expandable = FunctionExpandable;
    type Expanded = FunctionExpanded;
}
