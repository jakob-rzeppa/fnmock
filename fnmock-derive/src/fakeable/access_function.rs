use quote::quote;
use syn::parse_quote;

use crate::{ extract::item_impl::info::ImplItemFnInfo, fakeable::info::FakeableInfo };

pub fn generate_access_function_for_standalone(info: &FakeableInfo) -> syn::Result<syn::ItemFn> {
    let access_function_name = &info.access_function_name;
    let module_name = &info.module_name;
    let interface_struct_name = &info.interface_struct_name;

    let access_function_code = if let Some(generic_info) = &info.generic_info {
        let generic_types = generic_info.generic_types.as_slice();
        let generic_params = generic_info.generic_params.as_slice();

        quote! {
            #[cfg(test)]
            pub(crate) fn #access_function_name<#(#generic_params),*>() -> self::#module_name::#interface_struct_name<#(#generic_types),*> {
                self::#module_name::#interface_struct_name::new()
            }
        }
    } else {
        quote! {
            #[cfg(test)]
            pub(crate) fn #access_function_name() -> self::#module_name::#interface_struct_name {
                self::#module_name::#interface_struct_name::new()
            }
        }
    };

    syn::parse2(access_function_code)
}

/// Generates access methods for an impl block.
///
/// # Arguments
///
/// - `original_item_impl`: The `syn::ItemImpl` representing the original impl block. Should be a reference, so we can clone it and create a new impl block with the access methods.
/// - `fakeable_infos`: A slice of `FakeableInfo` structs, each representing a method in the impl block that is marked as fakeable.
/// - `item_impl_info`: A slice of `ImplItemFnInfo` structs, each representing a method in the impl block. This is used to extract information about the methods, such as their names and generics.
pub fn generate_access_methods_for_impl_block(
    original_item_impl: &syn::ItemImpl,
    fakeable_infos: &[FakeableInfo],
    item_impl_info: &[ImplItemFnInfo]
) -> syn::Result<syn::ItemImpl> {
    let access_methods: Vec<syn::ImplItemFn> = fakeable_infos
        .iter()
        .zip(item_impl_info.iter())
        .map(|(fakeable_info, item_impl_info)| {
            generate_access_method_for_impl_block(fakeable_info, item_impl_info)
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let mut access_item_impl = original_item_impl.clone();
    access_item_impl.items = access_methods.into_iter().map(syn::ImplItem::Fn).collect();

    Ok(parse_quote!(
        #[cfg(test)]
        #access_item_impl
    ))
}

fn generate_access_method_for_impl_block(
    fakeable_info: &FakeableInfo,
    item_impl_info: &ImplItemFnInfo
) -> syn::Result<syn::ImplItemFn> {
    let access_function_name = &fakeable_info.access_function_name;
    let module_name = &fakeable_info.module_name;
    let interface_struct_name = &fakeable_info.interface_struct_name;

    let access_method_code = if
        let (Some(fakeable_generic_info), Some(method_generic_info)) = (
            &fakeable_info.generic_info,
            &item_impl_info.generic_info,
        )
    {
        let generic_types = fakeable_generic_info.generic_types.as_slice();
        let method_generic_params = method_generic_info.method_type_params.as_slice();

        quote! {
            pub(crate) fn #access_function_name<#(#method_generic_params),*>() -> self::#module_name::#interface_struct_name<#(#generic_types),*> {
                self::#module_name::#interface_struct_name::new()
            }
        }
    } else {
        quote! {
            pub(crate) fn #access_function_name() -> self::#module_name::#interface_struct_name {
                self::#module_name::#interface_struct_name::new()
            }
        }
    };

    syn::parse2(access_method_code)
}
