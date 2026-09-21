pub mod from;
pub mod into;

pub struct ImplExpanded {
    impl_with_inline_calls: syn::ItemImpl,
    accessor_impl_block: syn::ItemImpl,
    modules: Vec<syn::ItemMod>,
}
