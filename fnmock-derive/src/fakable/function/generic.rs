use quote::quote;

pub fn insert_generic_fake_call_into_fn_block(
    fn_block: &syn::Block,
    fn_fake_name: &syn::Ident,
    fn_inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    fn_generics: &syn::Generics
) -> syn::Block {
    let input_idents: Vec<syn::Ident> = fn_inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    return Some(pat_ident.ident.clone());
                }
            }
            None
        })
        .collect();

    // Extract generic type parameters
    let generic_idents: Vec<_> = fn_generics.params
        .iter()
        .filter_map(|param| {
            if let syn::GenericParam::Type(type_param) = param {
                Some(&type_param.ident)
            } else {
                None
            }
        })
        .collect();

    let fake_call = if generic_idents.is_empty() {
        quote! {
            #[cfg(test)]
            if #fn_fake_name::is_set() {
                let impl_fn = #fn_fake_name::get();
                return impl_fn(#(#input_idents),*);
            }
        }
    } else {
        quote! {
            #[cfg(test)]
            if #fn_fake_name::is_set_for::<#(#generic_idents),*>() {
                let impl_fn = #fn_fake_name::get_for::<#(#generic_idents),*>();
                return impl_fn(#(#input_idents),*);
            }
        }
    };

    let fake_call_stmt: syn::Stmt = syn::parse(fake_call.into()).unwrap();

    let mut new_block = fn_block.clone();
    new_block.stmts.insert(0, fake_call_stmt);
    new_block
}

pub fn create_generic_fake_module(
    fn_name: &syn::Ident,
    fn_fake_name: &syn::Ident,
    fn_generics: &syn::Generics
) -> syn::ItemMod {
    // Extract generic type parameters with their bounds
    let generic_params: Vec<_> = fn_generics.params
        .iter()
        .filter_map(|param| {
            if let syn::GenericParam::Type(type_param) = param { Some(type_param) } else { None }
        })
        .collect();

    let generic_count = generic_params.len();

    // Extract just the type names for building TypeId array and function signatures
    let generic_idents: Vec<_> = generic_params
        .iter()
        .map(|param| &param.ident)
        .collect();

    // Build the generics with 'static bound added to all
    let generics_with_static: Vec<_> = generic_params
        .iter()
        .map(|param| {
            let ident = &param.ident;
            let bounds = &param.bounds;
            quote! { #ident: #bounds + 'static }
        })
        .collect();

    // Build TypeId array: [TypeId::of::<T>(), TypeId::of::<U>(), ...]
    let type_id_array: Vec<_> = generic_idents
        .iter()
        .map(|ident| {
            quote! { TypeId::of::<#ident>() }
        })
        .collect();

    let fake_module =
        quote! {
        #[cfg(test)]
        pub(crate) mod #fn_fake_name {
            use std::rc::Rc;
            use std::any::TypeId;

            use fnmock::generic_fake_store::GenericFakeStore;

            use super::*;

            thread_local! {
                static FAKE: std::cell::RefCell<GenericFakeStore<#generic_count>> = std::cell::RefCell::new(
                    GenericFakeStore::new(stringify!(#fn_name))
                );
            }

            pub(crate) fn setup<#(#generics_with_static),*>(function: fn(#(#generic_idents),*) -> String) {
                let generic_types = [#(#type_id_array),*];

                FAKE.with_borrow_mut(|fake| {
                    fake.setup_for(generic_types, function);
                });
            }

            pub(crate) fn clear() {
                FAKE.with_borrow_mut(|fake| {
                    fake.clear();
                })
            }

            pub(crate) fn clear_for<#(#generics_with_static),*>() {
                let generic_types = [#(#type_id_array),*];

                FAKE.with_borrow_mut(|fake| {
                    fake.clear_for(generic_types);
                })
            }

            pub(crate) fn is_set_for<#(#generics_with_static),*>() -> bool {
                let generic_types = [#(#type_id_array),*];

                FAKE.with_borrow(|fake| { fake.is_set_for(generic_types) })
            }

            pub(crate) fn get_for<#(#generics_with_static),*>() -> Rc<fn(#(#generic_idents),*) -> String> {
                let generic_types = [#(#type_id_array),*];

                FAKE.with_borrow(|fake| { fake.get_for::<fn(#(#generic_idents),*) -> String>(generic_types) })
            }
        }
    };

    syn::parse(fake_module.into()).unwrap()
}
