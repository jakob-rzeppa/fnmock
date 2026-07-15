use quote::quote;

use crate::fakeable::inline_call::info::InlineCallInfo;

/// Inserts an inline call to the fake implementation at the beginning of the original function block.
///
/// If the fake is set, it will call the fake implementation and return its result. Otherwise, it will execute the original function block.
pub fn insert_inline_call_into_fn_block(
    original_block: &syn::Block,
    inline_call_info: &InlineCallInfo
) -> syn::Block {
    let fake_call_values = &inline_call_info.fake_call_values;
    let module_name = &inline_call_info.module_name;
    let interface_struct_name = &inline_call_info.interface_struct_name;

    let new_block = if let Some(generic_types) = &inline_call_info.generic_types {
        quote! {
            {
                #[cfg(test)]
                {
                    let fake = self::#module_name::#interface_struct_name::<#(#generic_types),*>::new();
                    if fake.is_set() {
                        let implementation = fake.get();
                        return implementation(#(#fake_call_values),*);
                    }
                }

                #original_block
            }
        }
    } else {
        quote! {
            {
                #[cfg(test)]
                {
                    let fake = self::#module_name::#interface_struct_name::new();
                    if fake.is_set() {
                        let implementation = fake.get();
                        return implementation(#(#fake_call_values),*);
                    }
                }

                #original_block
            }
        }
    };

    syn::parse2(new_block).expect("Failed to parse the new block with inline call")
}
