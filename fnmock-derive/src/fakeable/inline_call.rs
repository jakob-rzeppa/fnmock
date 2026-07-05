use quote::quote;

/// Inserts an inline call to the fake implementation at the beginning of the original function block.
///
/// If the fake is set, it will call the fake implementation and return its result. Otherwise, it will execute the original function block.
pub fn insert_inline_call_into_fn_block(
    original_block: &syn::Block,
    param_idents: &[syn::Ident],
    module_name: &syn::Ident,
    interface_struct_name: &syn::Ident,
    generic_types: Option<&[syn::Type]>
) -> syn::Block {
    let new_block = if let Some(generic_types) = generic_types {
        quote! {
            {
                #[cfg(test)]
                {
                    let fake = self::#module_name::#interface_struct_name::<#(#generic_types),*>::new();
                    if fake.is_set() {
                        let implementation = fake.get();
                        return implementation(#(#param_idents),*);
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
                        return implementation(#(#param_idents),*);
                    }
                }

                #original_block
            }
        }
    };

    syn::parse(new_block.into()).expect("Failed to parse the new block with inline call")
}
