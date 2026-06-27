use quote::quote;

use crate::{ extract::impl_block::ItemImplMethodInfo, fakeable::info::FakeableInfo };

pub fn generate_access_function_for_standalone(info: &FakeableInfo) -> syn::Result<syn::ItemFn> {
    let access_function_name = &info.access_function_name;
    let module_name = &info.module_name;
    let interface_struct_name = &info.interface_struct_name;

    let access_function_code = if let Some(generic_info) = &info.generic_info {
        let generic_idents = generic_info.generic_idents.as_slice();
        let generic_params = generic_info.generic_params.as_slice();

        quote! {
            pub(crate) fn #access_function_name<#(#generic_params),*>() -> #module_name::#interface_struct_name<#(#generic_idents),*> {
                #module_name::#interface_struct_name::new()
            }
        }
    } else {
        quote! {
            pub(crate) fn #access_function_name() -> #module_name::#interface_struct_name {
                #module_name::#interface_struct_name::new()
            }
        }
    };

    syn::parse2(access_function_code)
}

pub fn generate_access_methods_for_impl_block(
    fakeable_infos: &[FakeableInfo],
    item_impl_info: &[ItemImplMethodInfo]
) -> syn::Result<syn::ItemImpl> {
    let access_methods: Vec<syn::ImplItemFn> = fakeable_infos
        .iter()
        .zip(item_impl_info.iter())
        .map(|(fakeable_info, item_impl_info)| {
            generate_access_method_for_impl_block(fakeable_info, item_impl_info)
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let struct_name = &item_impl_info.first().expect("No methods in impl block").struct_name;

    if
        let Some(generic_info) = item_impl_info
            .first()
            .expect("No methods in impl block")
            .generic_info.as_ref()
    {
        let struct_generics = &generic_info.struct_type_params;
        let struct_generic_idents = &generic_info.struct_idents;

        syn::parse2(
            quote! {
                impl<#(#struct_generics),*> #struct_name<#(#struct_generic_idents),*> {
                    #(#access_methods)*
                }
            }
        )
    } else {
        syn::parse2(
            quote! {
                impl #struct_name {
                    #(#access_methods)*
                }
            }
        )
    }
}

fn generate_access_method_for_impl_block(
    fakeable_info: &FakeableInfo,
    item_impl_info: &ItemImplMethodInfo
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
        let generic_idents = fakeable_generic_info.generic_idents.as_slice();
        let method_generic_params = method_generic_info.method_type_params.as_slice();

        quote! {
            pub(crate) fn #access_function_name<#(#method_generic_params),*>() -> #module_name::#interface_struct_name<#(#generic_idents),*> {
                #module_name::#interface_struct_name::new()
            }
        }
    } else {
        quote! {
            pub(crate) fn #access_function_name() -> #module_name::#interface_struct_name {
                #module_name::#interface_struct_name::new()
            }
        }
    };

    syn::parse2(access_method_code)
}
