//! Code generation for the accessors of an impl block's methods.

use quote::quote;
use syn::parse_quote;

use crate::fakeable::access_function::info::AccessFunctionInfo;

/// Generates access methods for an impl block.
///
/// The accessors go into a second impl block that mirrors the original's generics and self type,
/// rather than into the original itself, which keeps the user's block untouched apart from the
/// inline call injected into each method body.
///
/// # Errors
///
/// Returns an error if the generated code fails to parse, which would be a bug in fnmock.
///
/// # Arguments
///
/// - `original_item_impl`: The `syn::ItemImpl` representing the original impl block. Should be a reference, so we can clone it and create a new impl block with the access methods.
/// - `access_function_infos`: A slice of `AccessFunctionInfo` structs, one per method in the impl block that is marked as fakeable, in the same order as the methods appear in the impl block.
pub fn generate_access_methods_for_impl_block(
    original_item_impl: &syn::ItemImpl,
    access_function_infos: &[AccessFunctionInfo],
) -> syn::Result<syn::ItemImpl> {
    let access_methods: Vec<syn::ImplItemFn> = access_function_infos
        .iter()
        .map(generate_access_method_for_impl_block)
        .collect::<syn::Result<Vec<_>>>()?;

    let mut access_item_impl = original_item_impl.clone();
    access_item_impl.items = access_methods.into_iter().map(syn::ImplItem::Fn).collect();

    Ok(parse_quote!(
        #[cfg(test)]
        #access_item_impl
    ))
}

/// Generates an access method for a single method in an impl block.
fn generate_access_method_for_impl_block(
    info: &AccessFunctionInfo,
) -> syn::Result<syn::ImplItemFn> {
    let access_function_name = &info.access_function_name;
    let module_name = &info.module_name;
    let interface_struct_name = &info.interface_struct_name;

    let access_method_code = if let Some(generic_info) = &info.generic_info {
        let generic_idents = generic_info.generic_idents.as_slice();
        let generic_params = generic_info.generic_params.as_slice();

        quote! {
            pub(crate) fn #access_function_name<#(#generic_params),*>() -> self::#module_name::#interface_struct_name<#(#generic_idents),*> {
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
