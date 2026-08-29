use quote::quote;

use crate::{
    expandable::common::spy::module::matcher::build_marker_construct,
    scheme::common::generic_scheme::GenericScheme,
};

/// Builds the interface struct's `impl` block: `expect`/`expectf`/`expect_times`/`expect_once`/
/// `expect_never`/`assert`, plus the private `set_expectation` they share.
///
/// For a generic spy, every method is scoped to one combination of generic arguments — the ones
/// the interface value was obtained with, via `#interface_name::<T>()` — by routing every store
/// access through [`GenericSpyStore::with_store_mut`](fnmock::generic_spy_store::GenericSpyStore).
pub fn build_interface_impl(
    interface_name: &syn::Ident,
    store_name: &syn::Ident,
    matcher_name: &syn::Ident,
    display_name: &str,
    param_idents: &[syn::Ident],
    param_types: &[syn::Type],
    generic_scheme: Option<&GenericScheme>,
    generic_display_fragments: &[syn::Expr],
    supports_expect: bool,
) -> proc_macro2::TokenStream {
    let marker_construct = build_marker_construct(generic_scheme);
    let expectf_signature = quote! { Fn(#(&#param_types),*) -> bool };

    let expect_method = supports_expect.then(|| {
        let expect_params = param_idents.iter().zip(param_types).map(|(ident, ty)| {
            quote! { #ident: impl ::fnmock::Predicate<#ty> + 'static, }
        });
        let expect_construct_fields = param_idents
            .iter()
            .map(|ident| quote! { #ident: ::std::rc::Rc::new(#ident), });
        (expect_params, expect_construct_fields)
    });

    if let Some(generic_scheme) = generic_scheme {
        let generic_params = &generic_scheme.params;
        let generic_idents = &generic_scheme.idents;
        let generic_keys = &generic_scheme.keys;
        let matcher_type = quote! { #matcher_name<#(#generic_idents),*> };
        let instantiation_name = quote! {
            format!("{}::<{}>", #display_name, [#(#generic_display_fragments),*].join(", "))
        };
        let expect_method = expect_method.map(|(expect_params, expect_construct_fields)| {
            quote! {
                /// Expect calls whose arguments satisfy one predicate per parameter, for this
                /// combination of generic arguments.
                pub fn expect(
                    &self,
                    #(#expect_params)*
                ) -> ::fnmock::expectation_handle::ExpectationHandle<#matcher_type> {
                    self.set_expectation(#matcher_name::Predicates {
                        #(#expect_construct_fields)*
                        #marker_construct
                    })
                }
            }
        });

        quote! {
            impl<#(#generic_params),*> #interface_name<#(#generic_idents),*> {
                #expect_method

                /// Expect calls whose arguments satisfy `function`, for this combination of
                /// generic arguments.
                pub fn expectf(
                    &self,
                    function: impl #expectf_signature + 'static,
                ) -> ::fnmock::expectation_handle::ExpectationHandle<#matcher_type> {
                    self.set_expectation(#matcher_name::Function {
                        function: ::std::rc::Rc::new(function),
                        #marker_construct
                    })
                }

                /// Expect this many calls of this combination of generic arguments, whatever
                /// their arguments.
                pub fn expect_times(&self, call_range: impl Into<::fnmock::call_range::CallRange>) {
                    #store_name.with_borrow_mut(|store| {
                        store.with_store_mut::<#matcher_type, _>(
                            [#(#generic_keys),*],
                            || #instantiation_name,
                            |spy| spy.set_total_call_range(call_range.into()),
                        )
                    });
                }

                /// Expect exactly one matching call of this combination of generic arguments.
                pub fn expect_once(&self) {
                    self.expect_times(1);
                }

                /// Expect this combination of generic arguments not to be called at all.
                pub fn expect_never(&self) {
                    self.expect_times(0);
                }

                /// Assert every expectation set on this combination of generic arguments is
                /// fulfilled. Other instantiations are not checked; see the `_spy_all()`
                /// accessor to sweep every one at once.
                pub fn assert(&self) {
                    #store_name.with_borrow(|store| {
                        store.assert_for(&[#(#generic_keys),*]);
                    });
                }

                fn set_expectation(
                    &self,
                    matcher: #matcher_type,
                ) -> ::fnmock::expectation_handle::ExpectationHandle<#matcher_type> {
                    ::fnmock::expectation_handle::ExpectationHandle::new(
                        matcher,
                        #instantiation_name,
                        |expectation| {
                            #store_name.with_borrow_mut(|store| {
                                store.with_store_mut::<#matcher_type, _>(
                                    [#(#generic_keys),*],
                                    || #instantiation_name,
                                    |spy| spy.add_expectation(expectation),
                                )
                            });
                        },
                        |sequences| {
                            #store_name.with_borrow_mut(|store| {
                                store.with_store_mut::<#matcher_type, _>(
                                    [#(#generic_keys),*],
                                    || #instantiation_name,
                                    |spy| spy.add_sequences(sequences),
                                )
                            });
                        },
                    )
                }
            }
        }
    } else {
        let expect_method = expect_method.map(|(expect_params, expect_construct_fields)| {
            quote! {
                /// Expect calls whose arguments satisfy one predicate per parameter.
                pub fn expect(
                    &self,
                    #(#expect_params)*
                ) -> ::fnmock::expectation_handle::ExpectationHandle<#matcher_name> {
                    self.set_expectation(#matcher_name::Predicates {
                        #(#expect_construct_fields)*
                    })
                }
            }
        });

        quote! {
            impl #interface_name {
                #expect_method

                /// Expect calls whose arguments satisfy `function`.
                pub fn expectf(
                    &self,
                    function: impl #expectf_signature + 'static,
                ) -> ::fnmock::expectation_handle::ExpectationHandle<#matcher_name> {
                    self.set_expectation(#matcher_name::Function {
                        function: ::std::rc::Rc::new(function),
                    })
                }

                /// Expect this many calls, whatever their arguments.
                pub fn expect_times(&self, call_range: impl Into<::fnmock::call_range::CallRange>) {
                    #store_name.with_borrow_mut(|spy| spy.set_total_call_range(call_range.into()));
                }

                /// Expect exactly one call, whatever its arguments.
                pub fn expect_once(&self) {
                    self.expect_times(1);
                }

                /// Expect the function not to be called at all.
                pub fn expect_never(&self) {
                    self.expect_times(0);
                }

                /// Assert every expectation set on this spy is fulfilled.
                pub fn assert(&self) {
                    #store_name.with_borrow(|spy| spy.assert());
                }

                fn set_expectation(
                    &self,
                    matcher: #matcher_name,
                ) -> ::fnmock::expectation_handle::ExpectationHandle<#matcher_name> {
                    ::fnmock::expectation_handle::ExpectationHandle::new(
                        matcher,
                        #display_name,
                        |expectation| {
                            #store_name.with_borrow_mut(|spy| {
                                spy.add_expectation(expectation);
                            })
                        },
                        |sequences| {
                            #store_name.with_borrow_mut(|spy| {
                                spy.add_sequences(sequences);
                            })
                        },
                    )
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
    fn test_non_generic_zero_params() {
        let interface_name: syn::Ident = parse_quote!(PingSpyInterface);
        let store_name: syn::Ident = parse_quote!(PING_SPY_STORE);
        let matcher_name: syn::Ident = parse_quote!(PingMatcher);

        let res = build_interface_impl(
            &interface_name,
            &store_name,
            &matcher_name,
            "ping",
            &[],
            &[],
            None,
            &[],
            true,
        );

        let expected = quote! {
            impl PingSpyInterface {
                pub fn expect(
                    &self,
                ) -> ::fnmock::expectation_handle::ExpectationHandle<PingMatcher> {
                    self.set_expectation(PingMatcher::Predicates {
                    })
                }

                pub fn expectf(
                    &self,
                    function: impl Fn() -> bool + 'static,
                ) -> ::fnmock::expectation_handle::ExpectationHandle<PingMatcher> {
                    self.set_expectation(PingMatcher::Function {
                        function: ::std::rc::Rc::new(function),
                    })
                }

                pub fn expect_times(&self, call_range: impl Into<::fnmock::call_range::CallRange>) {
                    PING_SPY_STORE.with_borrow_mut(|spy| spy.set_total_call_range(call_range.into()));
                }

                pub fn expect_once(&self) {
                    self.expect_times(1);
                }

                pub fn expect_never(&self) {
                    self.expect_times(0);
                }

                pub fn assert(&self) {
                    PING_SPY_STORE.with_borrow(|spy| spy.assert());
                }

                fn set_expectation(
                    &self,
                    matcher: PingMatcher,
                ) -> ::fnmock::expectation_handle::ExpectationHandle<PingMatcher> {
                    ::fnmock::expectation_handle::ExpectationHandle::new(
                        matcher,
                        "ping",
                        |expectation| {
                            PING_SPY_STORE.with_borrow_mut(|spy| {
                                spy.add_expectation(expectation);
                            })
                        },
                        |sequences| {
                            PING_SPY_STORE.with_borrow_mut(|spy| {
                                spy.add_sequences(sequences);
                            })
                        },
                    )
                }
            }
        };

        assert_eq!(strip_doc_comments(res), strip_doc_comments(expected));
    }

    #[test]
    fn test_generic_single_param_routes_through_generic_spy_store() {
        let interface_name: syn::Ident = parse_quote!(FooSpyInterface);
        let store_name: syn::Ident = parse_quote!(FOO_SPY_STORE);
        let matcher_name: syn::Ident = parse_quote!(FooMatcher);
        let param_idents: Vec<syn::Ident> = vec![parse_quote!(a)];
        let param_types: Vec<syn::Type> = vec![parse_quote!(T)];
        let generic_scheme = GenericScheme {
            params: vec![parse_quote!(T: 'static)],
            idents: vec![parse_quote!(T)],
            idents_without_const_generics: vec![parse_quote!(T)],
            keys: vec![parse_quote! {
                ::fnmock::generic_fake_store::key::GenericKeyPart::Type(::std::any::TypeId::of::<T>())
            }],
        };
        let display_fragments: Vec<syn::Expr> =
            vec![parse_quote! { ::std::any::type_name::<T>().to_string() }];

        let res = build_interface_impl(
            &interface_name,
            &store_name,
            &matcher_name,
            "foo",
            &param_idents,
            &param_types,
            Some(&generic_scheme),
            &display_fragments,
            true,
        );

        let rendered = res.to_string();
        assert!(rendered.contains("impl < T : 'static > FooSpyInterface < T >"));
        assert!(rendered.contains("with_store_mut :: < FooMatcher < T > , _ >"));
        assert!(rendered.contains("_marker : :: std :: marker :: PhantomData ,"));
        assert!(rendered.contains("assert_for"));
        assert!(
            rendered.contains("GenericKeyPart :: Type (:: std :: any :: TypeId :: of :: < T > ())")
        );
    }

    /// The builder attaches doc comments (`///`) to the generated methods, which the exact
    /// token-stream comparison in `test_non_generic_zero_params` does not want to hand-duplicate.
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
