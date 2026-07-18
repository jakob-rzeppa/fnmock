//! Code generation for a free function's accessor.

use quote::quote;

use crate::fakeable::access_function::info::AccessFunctionInfo;

/// Generates the accessor for a free function, e.g. `fetch_user_fake()` next to `fetch_user`.
///
/// For a generic function the accessor is generic too, so that `fetch_user_fake::<i32>()` reaches
/// the fake keyed by exactly those generic arguments.
///
/// # Errors
///
/// Returns an error if the generated code fails to parse, which would be a bug in fnmock.
pub fn generate_access_function_for_standalone(
    info: &AccessFunctionInfo,
) -> syn::Result<syn::ItemFn> {
    let access_function_name = &info.access_function_name;
    let module_name = &info.module_name;
    let interface_struct_name = &info.interface_struct_name;

    let access_function_code = if let Some(generic_info) = &info.generic_info {
        let generic_idents = generic_info.generic_idents.as_slice();
        let generic_params = generic_info.generic_params.as_slice();

        quote! {
            #[cfg(test)]
            pub(crate) fn #access_function_name<#(#generic_params),*>() -> self::#module_name::#interface_struct_name<#(#generic_idents),*> {
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
