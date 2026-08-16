pub mod from;
pub mod into;

pub struct FunctionExpanded {
    fn_with_inline_call: syn::ItemFn,
    accessor_fn: syn::ItemFn,
    module: syn::ItemMod,
}
