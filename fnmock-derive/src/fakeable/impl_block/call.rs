use quote::quote;

use crate::fakeable::impl_block::info::FakeableImplFnInfo;

pub fn insert_call_into_fn_body(fn_body: &mut syn::Block, fn_info: &FakeableImplFnInfo) {
    let fake_access_fn_name = &fn_info.fake_access_fn_name;
    let fn_param_idents = &fn_info.fn_param_idents;
    let fn_generic_idents = fn_info.generic_info
        .as_ref()
        .map(|generic_info| generic_info.fn_generic_idents.clone())
        .unwrap_or(Vec::new());

    let call_expr =
        quote! {
        #[cfg(test)]
        if Self::#fake_access_fn_name::<#(#fn_generic_idents),*>().is_set() {
            let impl_fn = Self::#fake_access_fn_name::<#(#fn_generic_idents),*>().get();
            return impl_fn(#(#fn_param_idents),*);
        }
    };

    // Insert the call expression at the beginning of the function body
    fn_body.stmts.insert(0, syn::parse_quote! {
        #call_expr;
    });
}
