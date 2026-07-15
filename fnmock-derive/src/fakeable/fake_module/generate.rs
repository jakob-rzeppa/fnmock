use quote::{ quote };

use crate::{ fakeable::fake_module::info::FakeModuleInfo, module_builder::ModuleBuilder };

/// Generates the code for a fake module based on the provided FakeModuleInfo.
pub fn generate_fake_module_code(info: &FakeModuleInfo) -> syn::Result<syn::ItemMod> {
    if let Some(_) = &info.generic_info {
        generate_generic_fake_module_code(info)
    } else {
        generate_regular_fake_module_code(info)
    }
}

/// Generates the code for a regular (non-generic) fake module.
fn generate_regular_fake_module_code(info: &FakeModuleInfo) -> syn::Result<syn::ItemMod> {
    let module_name = &info.module_name;
    let store_name = &info.store_name;
    let display_name = &info.display_name;
    let interface_struct_name = &info.interface_struct_name;
    let fn_closure_trait = &info.fn_closure_trait;

    let mut module_builder = ModuleBuilder::new();

    module_builder.set_name(module_name.clone());

    module_builder.set_store(
        quote! {
            static #store_name: std::cell::RefCell<fnmock::fake_store::FakeStore<std::rc::Rc<dyn #fn_closure_trait>>> =
                std::cell::RefCell::new(fnmock::fake_store::FakeStore::new(stringify!(#display_name)));
        }
    );

    module_builder.set_interface_struct(
        quote! {
            pub(crate) struct #interface_struct_name;

            impl #interface_struct_name {
                pub(crate) fn new() -> Self {
                    Self
                }

                pub(crate) fn setup(self, function: impl #fn_closure_trait + 'static) -> Self {
                    #store_name.with(|store| {
                        store.borrow_mut().setup(std::rc::Rc::new(function));
                    });
                    self
                }

                pub(crate) fn clear(self) -> Self {
                    #store_name.with(|store| {
                        store.borrow_mut().clear();
                    });
                    self
                }

                pub(crate) fn is_set(&self) -> bool {
                    #store_name.with(|store| store.borrow().is_set())
                }

                pub(crate) fn get(&self) -> std::rc::Rc<dyn #fn_closure_trait> {
                    #store_name.with(|store| store.borrow().get())
                }
            }
        }
    );

    module_builder.build_module()
}

/// Generates the code for a generic fake module.
fn generate_generic_fake_module_code(info: &FakeModuleInfo) -> syn::Result<syn::ItemMod> {
    let module_name = &info.module_name;
    let store_name = &info.store_name;
    let display_name = &info.display_name;
    let interface_struct_name = &info.interface_struct_name;
    let fn_closure_trait = &info.fn_closure_trait;

    let (generic_count, generic_types, generic_params, generic_type_ids) = if
        let Some(generic_info) = &info.generic_info
    {
        (
            generic_info.generic_count,
            &generic_info.generic_types,
            &generic_info.generic_params,
            &generic_info.generic_type_ids,
        )
    } else {
        unreachable!(
            "generate_generic_fake_module_code should only be called when info.generic_info is Some"
        );
    };

    let generic_types_without_const_generics = generic_params.iter().filter_map(|param| {
        match param {
            syn::GenericParam::Type(type_param) => Some(type_param.ident.clone()),
            _ => None,
        }
    });

    let mut module_builder = ModuleBuilder::new();

    module_builder.set_name(module_name.clone());

    module_builder.set_store(
        quote! {
            static #store_name: std::cell::RefCell<fnmock::generic_fake_store::GenericFakeStore<#generic_count>> =
                std::cell::RefCell::new(
                    fnmock::generic_fake_store::GenericFakeStore::new(stringify!(#display_name))
                );
        }
    );

    module_builder.set_interface_struct(
        quote! {
            pub(crate) struct #interface_struct_name<#(#generic_params),*> {
                _marker: std::marker::PhantomData<(#(#generic_types_without_const_generics),*)>,
            }

            impl<#(#generic_params),*> #interface_struct_name<#(#generic_types),*> {
                pub(crate) fn new() -> Self {
                    Self {
                        _marker: std::marker::PhantomData,
                    }
                }

                pub(crate) fn setup(self, function: impl #fn_closure_trait + 'static) -> Self {
                    #store_name.with_borrow_mut(|fake| {
                        fake.setup_for::<Box<dyn #fn_closure_trait>>([#(#generic_type_ids),*], Box::new(function));
                    });
                    self
                }

                pub(crate) fn clear(self) -> Self {
                    #store_name.with_borrow_mut(|fake| {
                        fake.clear_for([#(#generic_type_ids),*]);
                    });
                    self
                }

                pub(crate) fn is_set(&self) -> bool {
                    #store_name.with_borrow(|fake| {
                        fake.is_set_for([#(#generic_type_ids),*])
                    })
                }

                pub(crate) fn get(&self) -> std::rc::Rc<Box<dyn #fn_closure_trait>> {
                    #store_name.with_borrow(|fake| {
                        fake.get_for::<Box<dyn #fn_closure_trait>>([#(#generic_type_ids),*])
                    })
                }
            }
        }
    );

    module_builder.build_module()
}
