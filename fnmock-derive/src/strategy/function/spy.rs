use crate::{
    expandable::function::FunctionExpandable, expanded::function::FunctionExpanded,
    item_info::function::info::FunctionInfo, plan::function::spy::FunctionSpyPlan,
    strategy::Strategy,
};

pub struct FunctionSpyStrategy;

impl Strategy for FunctionSpyStrategy {
    type Item = syn::ItemFn;
    type ItemInfo = FunctionInfo;
    type Plan = FunctionSpyPlan;
    type Expandable = FunctionExpandable;
    type Expanded = FunctionExpanded;
}
