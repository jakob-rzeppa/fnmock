use quote::quote;

pub fn build_interface_impl(
    interface_name: &syn::Ident,
    store_name: &syn::Ident,
    generic_params: Option<&[syn::GenericParam]>,
    generic_idents: Option<&[syn::Ident]>,
    generic_keys: Option<&[syn::Expr]>,
    fn_closure_trait: &syn::TraitBound,
) -> proc_macro2::TokenStream {
    if let (Some(generic_params), Some(generic_idents), Some(generic_keys)) =
        (generic_params, generic_idents, generic_keys)
    {
        quote! {
            impl<#(#generic_params),*> #interface_name<#(#generic_idents),*> {
                /// Install a fake implementation for this combination of generic arguments,
                /// replacing any previously set one.
                ///
                /// The closure mirrors the faked method's signature: same parameters (including
                /// destructuring patterns) and same return type. For an `async fn`, the closure is
                /// a plain synchronous closure returning the output type directly, not a future.
                /// Type parameters are keyed by `TypeId` and must be `'static`; const parameters
                /// are keyed by value, so e.g. a fake for `foo::<5>()` leaves `foo::<7>()` running
                /// the real body, and the const value isn't accessible inside the closure.
                pub fn setup(&self, function: impl #fn_closure_trait + 'static) {
                    #store_name.with_borrow_mut(|fake| {
                        fake.setup_for::<Box<dyn #fn_closure_trait>>([#(#generic_keys),*], Box::new(function));
                    });
                }

                /// Remove the fake implementation for this combination of generic arguments, so
                /// the real method body runs again.
                ///
                /// Fakes are thread-local and each `#[test]` runs on its own thread, so tests never
                /// leak fakes into each other — you don't need to call this between tests.
                pub fn clear(&self) {
                    #store_name.with_borrow_mut(|fake| {
                        fake.clear_for([#(#generic_keys),*]);
                    });
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
            }
        }
    } else {
        quote! {
            impl #interface_name {
                /// Install a fake implementation, replacing any previously set one.
                ///
                /// The closure mirrors the faked method's signature: same parameters (including
                /// destructuring patterns) and same return type. For an `async fn`, the closure is a
                /// plain synchronous closure returning the output type directly, not a future.
                pub fn setup(&self, function: impl #fn_closure_trait + 'static) {
                    #store_name.with(|store| {
                        store.borrow_mut().setup(::std::rc::Rc::new(function));
                    });
                }

                /// Remove the fake implementation, so the real method body runs again.
                ///
                /// Fakes are thread-local and each `#[test]` runs on its own thread, so tests never
                /// leak fakes into each other — you don't need to call this between tests.
                pub fn clear(&self) {
                    #store_name.with(|store| {
                        store.borrow_mut().clear();
                    });
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
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_non_generic() {
        let interface_name: syn::Ident = parse_quote!(MyMethodInterface);
        let store_name: syn::Ident = parse_quote!(MY_METHOD_STORE);
        let fn_closure_trait: syn::TraitBound = parse_quote!(Fn(i32) -> bool);

        let res = build_interface_impl(
            &interface_name,
            &store_name,
            None,
            None,
            None,
            &fn_closure_trait,
        );

        let expected = quote! {
            impl MyMethodInterface {
                pub fn setup(&self, function: impl Fn(i32) -> bool + 'static) {
                    MY_METHOD_STORE.with(|store| {
                        store.borrow_mut().setup(::std::rc::Rc::new(function));
                    });
                }

                pub fn clear(&self) {
                    MY_METHOD_STORE.with(|store| {
                        store.borrow_mut().clear();
                    });
                }

                pub fn is_set(&self) -> bool {
                    MY_METHOD_STORE.with(|store| store.borrow().is_set())
                }
            }
        };

        assert_eq!(strip_doc_comments(res), strip_doc_comments(expected));
    }

    #[test]
    fn test_generic_with_single_param() {
        let interface_name: syn::Ident = parse_quote!(MyMethodInterface);
        let store_name: syn::Ident = parse_quote!(MY_METHOD_STORE);
        let fn_closure_trait: syn::TraitBound = parse_quote!(Fn(i32) -> bool);
        let generic_params: Vec<syn::GenericParam> = vec![parse_quote!(T)];
        let generic_idents: Vec<syn::Ident> = vec![parse_quote!(T)];
        let generic_keys: Vec<syn::Expr> = vec![parse_quote!(::std::any::TypeId::of::<T>())];

        let res = build_interface_impl(
            &interface_name,
            &store_name,
            Some(&generic_params),
            Some(&generic_idents),
            Some(&generic_keys),
            &fn_closure_trait,
        );

        let expected = quote! {
            impl<T> MyMethodInterface<T> {
                pub fn setup(&self, function: impl Fn(i32) -> bool + 'static) {
                    MY_METHOD_STORE.with_borrow_mut(|fake| {
                        fake.setup_for::<Box<dyn Fn(i32) -> bool>>([::std::any::TypeId::of::<T>()], Box::new(function));
                    });
                }

                pub fn clear(&self) {
                    MY_METHOD_STORE.with_borrow_mut(|fake| {
                        fake.clear_for([::std::any::TypeId::of::<T>()]);
                    });
                }

                pub fn is_set(&self) -> bool {
                    MY_METHOD_STORE.with_borrow(|fake| {
                        fake.is_set_for([::std::any::TypeId::of::<T>()])
                    })
                }
            }
        };

        assert_eq!(strip_doc_comments(res), strip_doc_comments(expected));
    }

    #[test]
    fn test_generic_with_multiple_params_and_keys() {
        let interface_name: syn::Ident = parse_quote!(MyMethodInterface);
        let store_name: syn::Ident = parse_quote!(MY_METHOD_STORE);
        let fn_closure_trait: syn::TraitBound = parse_quote!(Fn());
        let generic_params: Vec<syn::GenericParam> =
            vec![parse_quote!(T), parse_quote!(const C: u32)];
        let generic_idents: Vec<syn::Ident> = vec![parse_quote!(T), parse_quote!(C)];
        let generic_keys: Vec<syn::Expr> =
            vec![parse_quote!(::std::any::TypeId::of::<T>()), parse_quote!(C)];

        let res = build_interface_impl(
            &interface_name,
            &store_name,
            Some(&generic_params),
            Some(&generic_idents),
            Some(&generic_keys),
            &fn_closure_trait,
        );

        let expected = quote! {
            impl<T, const C: u32> MyMethodInterface<T, C> {
                pub fn setup(&self, function: impl Fn() + 'static) {
                    MY_METHOD_STORE.with_borrow_mut(|fake| {
                        fake.setup_for::<Box<dyn Fn()>>([::std::any::TypeId::of::<T>(), C], Box::new(function));
                    });
                }

                pub fn clear(&self) {
                    MY_METHOD_STORE.with_borrow_mut(|fake| {
                        fake.clear_for([::std::any::TypeId::of::<T>(), C]);
                    });
                }

                pub fn is_set(&self) -> bool {
                    MY_METHOD_STORE.with_borrow(|fake| {
                        fake.is_set_for([::std::any::TypeId::of::<T>(), C])
                    })
                }
            }
        };

        assert_eq!(strip_doc_comments(res), strip_doc_comments(expected));
    }

    /// The builder attaches doc comments (`///`) to the generated methods, which we don't want
    /// to hand-duplicate in every expected token stream here. Doc comments lower to
    /// `#[doc = "..."]` attributes, so this filters those out before comparing, leaving the
    /// comparison focused on the generated code shape.
    fn strip_doc_comments(tokens: proc_macro2::TokenStream) -> String {
        strip_doc_comments_stream(tokens).to_string()
    }

    fn strip_doc_comments_stream(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        use proc_macro2::TokenTree;

        let mut filtered = proc_macro2::TokenStream::new();
        let mut iter = tokens.into_iter().peekable();
        while let Some(tt) = iter.next() {
            match &tt {
                TokenTree::Punct(p) if p.as_char() == '#' => {
                    if let Some(TokenTree::Group(group)) = iter.peek() {
                        let mut inner = group.stream().into_iter();
                        if let Some(TokenTree::Ident(ident)) = inner.next() {
                            if ident == "doc" {
                                iter.next();
                                continue;
                            }
                        }
                    }
                    filtered.extend(std::iter::once(tt));
                }
                TokenTree::Group(group) => {
                    let new_group = proc_macro2::Group::new(
                        group.delimiter(),
                        strip_doc_comments_stream(group.stream()),
                    );
                    filtered.extend(std::iter::once(TokenTree::Group(new_group)));
                }
                _ => filtered.extend(std::iter::once(tt)),
            }
        }
        filtered
    }
}
