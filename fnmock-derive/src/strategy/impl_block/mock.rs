use crate::{
    expandable::impl_block::ImplExpandable, expanded::impl_block::ImplExpanded,
    item_info::impl_block::ImplBlockInfo, scheme::mock::impl_block::ImplMockScheme,
    strategy::Strategy,
};

pub struct ImplMockStrategy;

impl Strategy for ImplMockStrategy {
    type Item = syn::ItemImpl;
    type ItemInfo = ImplBlockInfo;
    type Scheme = ImplMockScheme;
    type Expandable = ImplExpandable;
    type Expanded = ImplExpanded;
}
