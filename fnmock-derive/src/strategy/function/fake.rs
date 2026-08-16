use crate::{
    expandable::function::FunctionExpandable, expanded::function::FunctionExpanded,
    item_info::function::info::FunctionInfo, scheme::function::fake::FunctionFakeScheme,
    strategy::Strategy,
};

pub struct FunctionFakeStrategy;

impl Strategy for FunctionFakeStrategy {
    type Item = syn::ItemFn;
    type ItemInfo = FunctionInfo;
    type Scheme = FunctionFakeScheme;
    type Expandable = FunctionExpandable;
    type Expanded = FunctionExpanded;
}
