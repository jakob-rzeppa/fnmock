use crate::{
    expandable::impl_block::ImplExpandable, expanded::impl_block::ImplExpanded,
    item_info::impl_block::info::ImplBlockInfo, plan::impl_block::fake::ImplFakePlan,
    strategy::Strategy,
};

pub struct ImplFakeStrategy;

impl Strategy for ImplFakeStrategy {
    type Item = syn::ItemImpl;
    type ItemInfo = ImplBlockInfo;
    type Plan = ImplFakePlan;
    type Expandable = ImplExpandable;
    type Expanded = ImplExpanded;
}
