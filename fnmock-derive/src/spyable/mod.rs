//! Expansion of the `#[spyable]` attribute.
//!
//! The attribute expands into three pieces per spied function, one per submodule here:
//!
//! - `spy_module` — a `#[cfg(test)]` module holding the matcher, the thread-local store and the
//!   interface struct with `expect`/`expectf`/`expect_times`/`assert`.
//! - `inline_call` — the call recording injected at the top of the original body. Unlike a fake, a
//!   spy never replaces the body: it records the arguments and lets the real implementation run.
//! - `access_function` — the accessor tests call to reach the interface struct, e.g.
//!   `get_user_spy()`.

use quote::quote;

use crate::{
    extract::function::extract_function_info,
    names::NameType,
    spyable::{
        access_function::info::SpyAccessFunctionInfo, inline_call::info::SpyInlineCallInfo,
        spy_module::info::SpyModuleInfo,
    },
};

mod access_function;
mod inline_call;
mod spy_module;

/// Expand the `#[spyable]` attribute over the item it was applied to.
///
/// Takes `proc_macro2::TokenStream`s rather than `proc_macro::TokenStream`s so that expansion can
/// be exercised by ordinary unit tests; see the note at the ABI boundary in `lib.rs`.
///
/// # Errors
///
/// Returns a spanned error if the item is not a free function, if it fails to parse, or if the
/// signature cannot be spied on.
pub fn handle_spyable(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let expanded = match syn::parse2::<syn::Item>(item)? {
        syn::Item::Fn(mut item_fn) => {
            reject_generic_params(&item_fn.sig.generics)?;

            let function_info = extract_function_info(&item_fn, NameType::Spy)?;
            let spy_module_info = SpyModuleInfo::try_from(&function_info)?;
            let access_function_info = SpyAccessFunctionInfo::try_from(&function_info)?;
            let inline_call_info = SpyInlineCallInfo::try_from(&function_info)?;

            // Create the spy module code based on the extracted information
            let module = spy_module::generate::generate_spy_module_code(&spy_module_info)?;

            // Insert the call recording into the original function's block
            let modified_block = inline_call::generate::insert_spy_inline_call_into_fn_block(
                &item_fn.block,
                &inline_call_info,
            );
            *item_fn.block = modified_block;

            // Generate the access function
            let access_function =
                access_function::generate::generate_spy_access_function(&access_function_info)?;

            quote! {
                #item_fn

                #access_function

                #module
            }
        }
        syn::Item::Impl(item_impl) => {
            return Err(syn::Error::new_spanned(
                item_impl.self_ty,
                "The #[spyable] attribute does not support impl blocks yet. Only free functions can be spied on.",
            ));
        }
        item => {
            return Err(syn::Error::new_spanned(
                item,
                "The #[spyable] attribute can only be applied to functions.",
            ));
        }
    };

    Ok(expanded)
}

/// Rejects type and const generic parameters.
///
/// A spy keeps one store and one matcher per function, neither of which is keyed by — or able to
/// name — a generic argument, so a generic function would expand into a module referring to type
/// parameters that are not in scope there. Lifetime parameters are fine: the matcher borrows its
/// arguments under a lifetime of its own.
fn reject_generic_params(generics: &syn::Generics) -> syn::Result<()> {
    for param in &generics.params {
        match param {
            syn::GenericParam::Lifetime(_) => {}
            syn::GenericParam::Type(_) | syn::GenericParam::Const(_) => {
                return Err(syn::Error::new_spanned(
                    param,
                    "The #[spyable] attribute does not support generic type or const parameters. A spy keeps a single store per function, which is not keyed by generic arguments.",
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expand(item: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
        handle_spyable(quote! {}, item)
    }

    #[test]
    fn test_free_function_expands_into_the_function_the_accessor_and_the_module() {
        let expanded = expand(quote! {
            pub fn get_user(mut id: String, uuid: &str) -> String {
                id.push_str(uuid);
                id
            }
        })
        .expect("a plain free function should be spyable");

        let file: syn::File =
            syn::parse2(expanded).expect("the expansion should parse as a sequence of items");

        assert_eq!(
            file.items.len(),
            3,
            "expected the function, its accessor and the spy module"
        );
        assert!(matches!(file.items[0], syn::Item::Fn(_)));
        assert!(matches!(file.items[1], syn::Item::Fn(_)));
        assert!(matches!(file.items[2], syn::Item::Mod(_)));
    }

    /// The spy has to leave the original body in place — it observes calls, it does not replace
    /// them.
    #[test]
    fn test_the_original_body_is_kept_and_the_call_is_recorded_before_it() {
        let expanded = expand(quote! {
            fn save_user(id: String) {
                let _ = id;
            }
        })
        .expect("a plain free function should be spyable");

        let rendered = expanded.to_string();
        assert!(
            rendered.contains("save_user_spy_module :: internal_record_call (& (& id ,))"),
            "expected the recorded call to borrow `id` as a 1-tuple, got: {rendered}"
        );
        assert!(
            rendered.contains("let _ = id"),
            "expected the user's own body to survive expansion, got: {rendered}"
        );
    }

    #[test]
    fn test_impl_block_is_rejected() {
        let result = expand(quote! {
            impl UserService {
                fn get_user(&self, id: u32) -> String {
                    todo!()
                }
            }
        });

        let Err(error) = result else {
            panic!("impl blocks are not supported yet and should be rejected");
        };
        assert!(
            error.to_string().contains("impl blocks"),
            "the error should say impl blocks are unsupported, got: {error}"
        );
    }

    #[test]
    fn test_struct_is_rejected() {
        let result = expand(quote! {
            struct User {
                id: u32,
            }
        });

        assert!(
            result.is_err(),
            "the attribute should only be accepted on functions"
        );
    }

    #[test]
    fn test_generic_function_is_rejected() {
        let result = expand(quote! {
            fn get_user<T>(id: T) -> T {
                id
            }
        });

        let Err(error) = result else {
            panic!("a generic function should be rejected: a spy's store is not keyed by generics");
        };
        assert!(
            error.to_string().contains("generic"),
            "the error should mention generics, got: {error}"
        );
    }

    #[test]
    fn test_const_generic_function_is_rejected() {
        let result = expand(quote! {
            fn get_user<const N: usize>(ids: [u32; N]) {}
        });

        assert!(
            result.is_err(),
            "a const generic function should be rejected"
        );
    }

    /// Lifetimes are not part of what a spy keys on, and the matcher borrows under a lifetime of
    /// its own, so an explicit lifetime parameter is harmless.
    #[test]
    fn test_lifetime_only_generics_are_accepted() {
        let result = expand(quote! {
            fn get_user<'a>(id: &'a str) -> &'a str {
                id
            }
        });

        assert!(
            result.is_ok(),
            "a function generic only over lifetimes should be spyable"
        );
    }

    #[test]
    fn test_destructuring_param_is_rejected() {
        let result = expand(quote! {
            fn get_user((a, b): (i32, i32)) {}
        });

        assert!(
            result.is_err(),
            "a destructuring parameter should be rejected: a matcher needs one name per parameter"
        );
    }
}
