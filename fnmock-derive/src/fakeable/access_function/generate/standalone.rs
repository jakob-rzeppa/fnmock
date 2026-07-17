use quote::quote;

use crate::fakeable::access_function::info::AccessFunctionInfo;

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
