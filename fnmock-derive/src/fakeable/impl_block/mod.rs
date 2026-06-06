use quote::quote;

use crate::fakeable::impl_block::{
    call::insert_call_into_fn_body,
    extraction::extract_generic_fakeable_impl_fn_info,
    fake_module::build_fake_module,
};

mod fake_module;
mod info;
mod call;
mod extraction;
mod access_fn;

pub fn fakeable_impl_block(mut item_impl: syn::ItemImpl) -> syn::Result<proc_macro2::TokenStream> {
    let fakeable_info = extract_generic_fakeable_impl_fn_info(&item_impl)?;

    let fake_module = build_fake_module(&fakeable_info)?;

    item_impl.items.iter_mut().for_each(|item| {
        if let syn::ImplItem::Fn(impl_fn) = item {
            if
                let Some(fn_info) = fakeable_info
                    .iter()
                    .find(|info| info.fn_name == impl_fn.sig.ident)
            {
                insert_call_into_fn_body(&mut impl_fn.block, fn_info);
            }
        }
    });

    let access_fns: Vec<_> = fakeable_info
        .iter()
        .map(|fn_info| access_fn::build_access_fn(fn_info))
        .collect();

    // Add access functions as impl items
    for access_fn_tokens in access_fns {
        item_impl.items.push(syn::ImplItem::Fn(access_fn_tokens));
    }

    Ok(quote! {
        #item_impl

        #fake_module
    })
}
