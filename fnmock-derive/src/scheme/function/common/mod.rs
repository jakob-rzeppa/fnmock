use crate::item_info::original::OriginalFn;

pub struct FunctionCommonScheme {
    pub vis: syn::Visibility,

    pub original: OriginalFn,

    pub module_name: syn::Ident,
    pub display_name: String,
    pub accessor_name: syn::Ident,
}
