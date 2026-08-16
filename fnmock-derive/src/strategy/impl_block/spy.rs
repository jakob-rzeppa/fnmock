use crate::{
    expandable::impl_block::ImplExpandable, expanded::impl_block::ImplExpanded,
    item_info::impl_block::info::ImplBlockInfo, plan::impl_block::spy::ImplSpyPlan,
    strategy::Strategy,
};

pub struct ImplSpyStrategy;

impl Strategy for ImplSpyStrategy {
    type Item = syn::ItemImpl;
    type ItemInfo = ImplBlockInfo;
    type Plan = ImplSpyPlan;
    type Expandable = ImplExpandable;
    type Expanded = ImplExpanded;
}
