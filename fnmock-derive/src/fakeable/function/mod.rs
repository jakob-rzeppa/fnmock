use quote::quote;

mod regular;
mod generic;
mod info;
mod extraction;
mod access_fn;
mod fake_module;
mod call;

pub fn fakeable_function(item_fn: syn::ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let info = extraction::extract_fakeable_fn_info(&item_fn)?;

    // Generate the access function and fake module based on the extracted information
    let access_fn = access_fn::build_access_function(&info);
    let fake_module = fake_module::build_fake_module(&info);

    // Insert the call to the fake implementation at the beginning of the function body
    let mut modified_item_fn = item_fn.clone();
    call::insert_call_into_fn_body(&mut modified_item_fn.block, &info);

    Ok(quote! {
        #modified_item_fn

        #access_fn

        #fake_module
    })
}
