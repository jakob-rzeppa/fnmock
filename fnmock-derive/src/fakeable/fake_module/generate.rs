//! Code generation for a fake module.

use quote::quote;

use crate::{fakeable::fake_module::info::FakeModuleInfo, module_builder::ModuleBuilder};

/// Generates the code for a fake module based on the provided FakeModuleInfo.
///
/// Dispatches on whether the faked function is generic: a non-generic function needs only one
/// stored closure, while a generic one needs one per combination of generic arguments.
///
/// # Errors
///
/// Returns an error if the generated module fails to parse, which would be a bug in fnmock.
pub fn generate_fake_module_code(info: &FakeModuleInfo) -> syn::Result<syn::ItemMod> {
    if info.generic_info.is_some() {
        generate_generic_fake_module_code(info)
    } else {
        generate_regular_fake_module_code(info)
    }
}

/// Generates the code for a regular (non-generic) fake module.
fn generate_regular_fake_module_code(info: &FakeModuleInfo) -> syn::Result<syn::ItemMod> {
    let store_name = &info.store_name;
    let display_name = &info.display_name;
    let interface_struct_name = &info.interface_struct_name;
    let fn_closure_trait = &info.fn_closure_trait;

    let mut module_builder = ModuleBuilder::new();

    module_builder.set_name(info.module_name.clone());
    module_builder.set_visibility(info.visibility.clone());

    module_builder.set_store(
        quote! {
            static #store_name: ::std::cell::RefCell<::fnmock::fake_store::FakeStore<::std::rc::Rc<dyn #fn_closure_trait>>> =
                ::std::cell::RefCell::new(::fnmock::fake_store::FakeStore::new(#display_name));
        }
    );

    module_builder.add_part(quote! {
        pub struct #interface_struct_name;

        impl #interface_struct_name {
            /// Only called by the generated accessor function. Users should not call this directly.
            pub fn new() -> Self {
                Self
            }

            /// Install a fake implementation, replacing any previously set one.
            ///
            /// The closure mirrors the faked function's signature: same parameters (including
            /// destructuring patterns) and same return type. For an `async fn`, the closure is a
            /// plain synchronous closure returning the output type directly, not a future.
            pub fn setup(self, function: impl #fn_closure_trait + 'static) -> Self {
                #store_name.with(|store| {
                    store.borrow_mut().setup(::std::rc::Rc::new(function));
                });
                self
            }

            /// Remove the fake implementation, so the real function body runs again.
            ///
            /// Fakes are thread-local and each `#[test]` runs on its own thread, so tests never
            /// leak fakes into each other — you don't need to call this between tests.
            pub fn clear(self) -> Self {
                #store_name.with(|store| {
                    store.borrow_mut().clear();
                });
                self
            }

            /// Check whether a fake implementation is currently set.
            ///
            /// Fakes are thread-local, so this only reports the state on the calling thread. This
            /// is useful for confirming a fake reached code that may have crossed a thread
            /// boundary (e.g. via `tokio::spawn` or `std::thread::spawn`), since an unset fake
            /// falls through to the real implementation silently rather than erroring.
            pub fn is_set(&self) -> bool {
                #store_name.with(|store| store.borrow().is_set())
            }

            /// DO NOT USE THIS DIRECTLY.
            /// Only called by the macro-injected fake lookup in the faked function's body.
            /// Returns the fake implementation for the function.
            pub fn get(&self) -> ::std::rc::Rc<dyn #fn_closure_trait> {
                #store_name.with(|store| store.borrow().get())
            }
        }
    });

    module_builder.build_module()
}

/// Generates the code for a generic fake module.
fn generate_generic_fake_module_code(info: &FakeModuleInfo) -> syn::Result<syn::ItemMod> {
    let store_name = &info.store_name;
    let display_name = &info.display_name;
    let interface_struct_name = &info.interface_struct_name;
    let fn_closure_trait = &info.fn_closure_trait;

    let (generic_count, generic_types, generic_params, generic_keys) = if let Some(generic_info) =
        &info.generic_info
    {
        (
            generic_info.generic_count,
            &generic_info.generic_idents,
            &generic_info.generic_params,
            &generic_info.generic_keys,
        )
    } else {
        unreachable!(
            "generate_generic_fake_module_code should only be called when info.generic_info is Some"
        );
    };

    let generic_types_without_const_generics =
        generic_params.iter().filter_map(|param| match param {
            syn::GenericParam::Type(type_param) => Some(type_param.ident.clone()),
            _ => None,
        });

    let mut module_builder = ModuleBuilder::new();

    module_builder.set_name(info.module_name.clone());
    module_builder.set_visibility(info.visibility.clone());

    module_builder.set_store(
        quote! {
            static #store_name: ::std::cell::RefCell<::fnmock::generic_fake_store::GenericFakeStore<#generic_count>> =
                ::std::cell::RefCell::new(
                    ::fnmock::generic_fake_store::GenericFakeStore::new(#display_name)
                );
        }
    );

    module_builder.add_part(
        quote! {
            pub struct #interface_struct_name<#(#generic_params),*> {
                _marker: ::std::marker::PhantomData<(#(#generic_types_without_const_generics),*)>,
            }

            impl<#(#generic_params),*> #interface_struct_name<#(#generic_types),*> {
                /// Only called by the generated accessor function. Users should not call this directly.
                pub fn new() -> Self {
                    Self {
                        _marker: ::std::marker::PhantomData,
                    }
                }

                /// Install a fake implementation for this combination of generic arguments,
                /// replacing any previously set one.
                ///
                /// The closure mirrors the faked function's signature: same parameters (including
                /// destructuring patterns) and same return type. For an `async fn`, the closure is
                /// a plain synchronous closure returning the output type directly, not a future.
                /// Type parameters are keyed by `TypeId` and must be `'static`; const parameters
                /// are keyed by value, so e.g. a fake for `foo::<5>()` leaves `foo::<7>()` running
                /// the real body, and the const value isn't accessible inside the closure.
                pub fn setup(self, function: impl #fn_closure_trait + 'static) -> Self {
                    #store_name.with_borrow_mut(|fake| {
                        fake.setup_for::<Box<dyn #fn_closure_trait>>([#(#generic_keys),*], Box::new(function));
                    });
                    self
                }

                /// Remove the fake implementation for this combination of generic arguments, so
                /// the real function body runs again.
                ///
                /// Fakes are thread-local and each `#[test]` runs on its own thread, so tests never
                /// leak fakes into each other — you don't need to call this between tests.
                pub fn clear(self) -> Self {
                    #store_name.with_borrow_mut(|fake| {
                        fake.clear_for([#(#generic_keys),*]);
                    });
                    self
                }

                /// Check whether a fake implementation is currently set for this combination of
                /// generic arguments.
                ///
                /// Fakes are thread-local, so this only reports the state on the calling thread.
                /// This is useful for confirming a fake reached code that may have crossed a
                /// thread boundary (e.g. via `tokio::spawn` or `std::thread::spawn`), since an
                /// unset fake falls through to the real implementation silently rather than
                /// erroring.
                pub fn is_set(&self) -> bool {
                    #store_name.with_borrow(|fake| {
                        fake.is_set_for([#(#generic_keys),*])
                    })
                }

                /// DO NOT USE THIS DIRECTLY.
                /// Only called by the macro-injected fake lookup in the faked function's body.
                /// Returns the fake implementation for the function.
                pub fn get(&self) -> ::std::rc::Rc<Box<dyn #fn_closure_trait>> {
                    #store_name.with_borrow(|fake| {
                        fake.get_for::<Box<dyn #fn_closure_trait>>([#(#generic_keys),*])
                    })
                }
            }
        }
    );

    module_builder.build_module()
}
