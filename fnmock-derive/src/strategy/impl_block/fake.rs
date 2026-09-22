use crate::{
    expandable::impl_block::ImplExpandable, expanded::impl_block::ImplExpanded,
    item_info::impl_block::ImplBlockInfo, scheme::fake::impl_block::ImplFakeScheme,
    strategy::Strategy,
};

pub struct ImplFakeStrategy;

impl Strategy for ImplFakeStrategy {
    type Item = syn::ItemImpl;
    type ItemInfo = ImplBlockInfo;
    type Scheme = ImplFakeScheme;
    type Expandable = ImplExpandable;
    type Expanded = ImplExpanded;
}
