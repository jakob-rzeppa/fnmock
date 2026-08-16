use crate::{
    expandable::impl_block::ImplExpandable, expanded::impl_block::ImplExpanded,
    item_info::impl_block::info::ImplBlockInfo, scheme::impl_block::spy::ImplSpyScheme,
    strategy::Strategy,
};

pub struct ImplSpyStrategy;

impl Strategy for ImplSpyStrategy {
    type Item = syn::ItemImpl;
    type ItemInfo = ImplBlockInfo;
    type Scheme = ImplSpyScheme;
    type Expandable = ImplExpandable;
    type Expanded = ImplExpanded;
}
