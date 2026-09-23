use quote::quote;

use crate::scheme::common::generic_scheme::GenericScheme;

/// Builds the mock's own `impl` block holding the one `clear` that resets both halves.
///
/// The fake's and the spy's `clear` are suppressed on a mock (`include_clear == false`) because two
/// inherent methods of one name on the same struct don't compile. The two statements are separate
/// borrows of separate `thread_local`s, so a fake closure that calls `clear()` on its own mock
/// doesn't double-borrow.
pub fn build_mock_clear(
    interface_name: &syn::Ident,
    fake_store_name: &syn::Ident,
    spy_store_name: &syn::Ident,
    generic_scheme: Option<&GenericScheme>,
) -> proc_macro2::TokenStream {
    if let Some(generic_scheme) = generic_scheme {
        let generic_params = &generic_scheme.params;
        let generic_idents = &generic_scheme.idents;
        let generic_keys = &generic_scheme.keys;
        quote! {
            impl<#(#generic_params),*> #interface_name<#(#generic_idents),*> {
                /// Reset this combination of generic arguments to a freshly created state: remove
                /// the fake implementation, and drop every expectation, `expect_times` range and
                /// recorded call. Other instantiations are not affected.
                pub fn clear(&self) {
                    #fake_store_name.with_borrow_mut(|fake| {
                        fake.clear_for([#(#generic_keys),*]);
                    });
                    #spy_store_name.with_borrow_mut(|store| {
                        store.clear_for(&[#(#generic_keys),*]);
                    });
                }
            }
        }
    } else {
        quote! {
            impl #interface_name {
                /// Reset this mock to a freshly created state: remove the fake implementation, and
                /// drop every expectation, `expect_times` range and recorded call.
                pub fn clear(&self) {
                    #fake_store_name.with(|store| {
                        store.borrow_mut().clear();
                    });
                    #spy_store_name.with_borrow_mut(|spy| spy.clear());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_generic_clears_both_stores() {
        let interface_name: syn::Ident = syn::parse_quote!(PingMockInterface);
        let fake_store: syn::Ident = syn::parse_quote!(PING_FAKE_STORE);
        let spy_store: syn::Ident = syn::parse_quote!(PING_SPY_STORE);

        let tokens = build_mock_clear(&interface_name, &fake_store, &spy_store, None);

        let expected = quote! {
            impl PingMockInterface {
                /// Reset this mock to a freshly created state: remove the fake implementation, and
                /// drop every expectation, `expect_times` range and recorded call.
                pub fn clear(&self) {
                    PING_FAKE_STORE.with(|store| {
                        store.borrow_mut().clear();
                    });
                    PING_SPY_STORE.with_borrow_mut(|spy| spy.clear());
                }
            }
        };
        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
