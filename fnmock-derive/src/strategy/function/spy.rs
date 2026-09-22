use crate::{
    expandable::function::FunctionExpandable, expanded::function::FunctionExpanded,
    item_info::function::FunctionInfo, scheme::spy::function::FunctionSpyScheme,
    strategy::Strategy,
};

pub struct FunctionSpyStrategy;

impl Strategy for FunctionSpyStrategy {
    type Item = syn::ItemFn;
    type ItemInfo = FunctionInfo;
    type Scheme = FunctionSpyScheme;
    type Expandable = FunctionExpandable;
    type Expanded = FunctionExpanded;
}
