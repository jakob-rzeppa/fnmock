//! Code generation for a spy module.

use quote::quote;

use crate::{
    module_builder::ModuleBuilder,
    spyable::spy_module::{
        helpers::{
            build_expect_construct_fields, build_expect_params, build_matcher_function_matches_arm,
            build_matcher_function_signature, build_matcher_predicates_display_arm,
            build_matcher_predicates_fields, build_matcher_predicates_matches_arm,
            build_param_reference_tuple_type,
        },
        info::SpyModuleInfo,
    },
};

/// Generates the code for a spy module based on the provided SpyModuleInfo.
///
/// Every hole that depends on the spied function's parameter list is already fully rendered in
/// `info`, so this only substitutes fields into a fixed template.
///
/// # Errors
///
/// Returns an error if the generated module fails to parse, which would be a bug in fnmock.
pub fn generate_spy_module_code(info: &SpyModuleInfo) -> syn::Result<syn::ItemMod> {
    let store_name = &info.store_name;
    let display_name = &info.display_name;
    let matcher_name = &info.matcher_name;
    let interface_struct_name = &info.interface_struct_name;
    let param_types_unreferenced = &info.param_types_unreferenced;
    let param_idents = &info.param_idents;
    let params_tuple_type_named =
        build_param_reference_tuple_type(param_types_unreferenced, quote! { 'a });
    let params_tuple_type_elided =
        build_param_reference_tuple_type(param_types_unreferenced, quote! {});
    let matcher_predicates_fields =
        build_matcher_predicates_fields(param_idents, param_types_unreferenced);
    let matcher_function_signature = build_matcher_function_signature(param_types_unreferenced);
    let matcher_predicates_matches_arm = build_matcher_predicates_matches_arm(param_idents);
    let matcher_function_matches_arm = build_matcher_function_matches_arm(param_idents);
    let matcher_predicates_display_arm = build_matcher_predicates_display_arm(param_idents);
    let expect_params = build_expect_params(param_idents, param_types_unreferenced);
    let expect_construct_fields = build_expect_construct_fields(param_idents);

    let mut builder = ModuleBuilder::new();

    builder.set_name(info.module_name.clone());

    builder.set_store(quote! {
        static #store_name: ::std::cell::RefCell<::fnmock::spy_store::SpyStore<#matcher_name>> =
            ::std::cell::RefCell::new(
                ::fnmock::spy_store::SpyStore::new(#display_name)
            );
    });

    // Matcher
    builder.add_part(quote! {
        #[derive(Clone)]
        pub enum #matcher_name {
            Predicates {
                #matcher_predicates_fields
            },
            Function {
                function: ::std::rc::Rc<dyn #matcher_function_signature>,
            },
        }

        impl ::fnmock::matcher::Matcher for #matcher_name {
            type Params<'a> = #params_tuple_type_named;

            fn matches(&self, params: &Self::Params<'_>) -> bool {
                match self {
                    #matcher_predicates_matches_arm,
                    #matcher_function_matches_arm,
                }
            }
        }

        impl ::std::fmt::Display for #matcher_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    #matcher_predicates_display_arm,
                    Self::Function { .. } => {
                        write!(f, "a function predicate")
                    }
                }
            }
        }
    });

    // Internals
    builder.add_part(quote! {
        pub(super) fn internal_record_call(params: &#params_tuple_type_elided) {
            #store_name.with_borrow_mut(|spy| {
                spy.record_call(params);
            })
        }

        pub(super) fn internal_get_interface() -> #interface_struct_name {
            #interface_struct_name {}
        }
    });

    // Interface
    builder.add_part(quote! {
        pub struct #interface_struct_name {}

        impl #interface_struct_name {
            /// Expect calls whose arguments satisfy one predicate per parameter.
            pub fn expect(
                &self,
                #expect_params
            ) -> ::fnmock::expectation_handle::ExpectationHandle<#matcher_name> {
                self.set_expectation(#matcher_name::Predicates {
                    #expect_construct_fields
                })
            }

            /// Expect calls whose arguments satisfy `function`.
            pub fn expectf(
                &self,
                function: impl #matcher_function_signature + 'static,
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

            /// Record a call. Called by the spied function, not by the test.
            pub fn record_call(&self, params: &#params_tuple_type_elided) {
                #store_name.with_borrow_mut(|spy| spy.record_call(params));
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
    });

    builder.build_module()
}
#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    #[test]
    fn test_generate_spy_module_code_for_multiple_params() {
        let info = SpyModuleInfo {
            module_name: syn::parse_str("get_user_spy_module").unwrap(),
            store_name: syn::parse_str("SPY").unwrap(),
            display_name: "get_user".to_string(),
            matcher_name: syn::parse_str("GetUserMatcher").unwrap(),
            param_idents: vec![
                syn::parse_str("id").unwrap(),
                syn::parse_str("uuid").unwrap(),
            ],
            param_types_unreferenced: vec![
                syn::parse_str("String").unwrap(),
                syn::parse_str("str").unwrap(),
            ],
            interface_struct_name: syn::parse_str("GetUserSpyInterface").unwrap(),
        };

        let generated = generate_spy_module_code(&info).expect("generated module should parse");

        let expected: syn::ItemMod = syn::parse_quote! {
            #[cfg(test)]
            pub(crate) mod get_user_spy_module {
                use super::*;

                thread_local! {
                    static SPY: ::std::cell::RefCell<::fnmock::spy_store::SpyStore<GetUserMatcher>> =
                        ::std::cell::RefCell::new(
                            ::fnmock::spy_store::SpyStore::new("get_user")
                        );
                }

                #[derive(Clone)]
                pub enum GetUserMatcher {
                    Predicates {
                        id: ::std::rc::Rc<dyn ::fnmock::Predicate<String>>,
                        uuid: ::std::rc::Rc<dyn ::fnmock::Predicate<str>>,
                    },
                    Function {
                        function: ::std::rc::Rc<dyn Fn(&String, &str) -> bool>,
                    },
                }

                impl ::fnmock::matcher::Matcher for GetUserMatcher {
                    type Params<'a> = (&'a String, &'a str,);

                    fn matches(&self, params: &Self::Params<'_>) -> bool {
                        match self {
                            Self::Predicates { id, uuid } => id.eval(params.0) && uuid.eval(params.1),
                            Self::Function { function } => function(params.0, params.1),
                        }
                    }
                }

                impl ::std::fmt::Display for GetUserMatcher {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        match self {
                            Self::Predicates { id, uuid } => {
                                write!(
                                    f,
                                    "{} && {}",
                                    id.to_string().replacen("var", "id", 1),
                                    uuid.to_string().replacen("var", "uuid", 1)
                                )
                            },
                            Self::Function { .. } => {
                                write!(f, "a function predicate")
                            }
                        }
                    }
                }

                pub(super) fn internal_record_call(params: &(&String, &str,)) {
                    SPY.with_borrow_mut(|spy| {
                        spy.record_call(params);
                    })
                }

                pub(super) fn internal_get_interface() -> GetUserSpyInterface {
                    GetUserSpyInterface {}
                }

                pub struct GetUserSpyInterface {}

                impl GetUserSpyInterface {
                    /// Expect calls whose arguments satisfy one predicate per parameter.
                    pub fn expect(
                        &self,
                        id: impl ::fnmock::Predicate<String> + 'static,
                        uuid: impl ::fnmock::Predicate<str> + 'static,
                    ) -> ::fnmock::expectation_handle::ExpectationHandle<GetUserMatcher> {
                        self.set_expectation(GetUserMatcher::Predicates {
                            id: ::std::rc::Rc::new(id),
                            uuid: ::std::rc::Rc::new(uuid),
                        })
                    }

                    /// Expect calls whose arguments satisfy `function`.
                    pub fn expectf(
                        &self,
                        function: impl Fn(&String, &str) -> bool + 'static,
                    ) -> ::fnmock::expectation_handle::ExpectationHandle<GetUserMatcher> {
                        self.set_expectation(GetUserMatcher::Function {
                            function: ::std::rc::Rc::new(function),
                        })
                    }

                    /// Expect this many calls, whatever their arguments.
                    pub fn expect_times(&self, call_range: impl Into<::fnmock::call_range::CallRange>) {
                        SPY.with_borrow_mut(|spy| spy.set_total_call_range(call_range.into()));
                    }

                    /// Expect exactly one call, whatever its arguments.
                    pub fn expect_once(&self) {
                        self.expect_times(1);
                    }

                    /// Expect the function not to be called at all.
                    pub fn expect_never(&self) {
                        self.expect_times(0);
                    }

                    /// Record a call. Called by the spied function, not by the test.
                    pub fn record_call(&self, params: &(&String, &str,)) {
                        SPY.with_borrow_mut(|spy| spy.record_call(params));
                    }

                    /// Assert every expectation set on this spy is fulfilled.
                    pub fn assert(&self) {
                        SPY.with_borrow(|spy| spy.assert());
                    }

                    fn set_expectation(
                        &self,
                        matcher: GetUserMatcher,
                    ) -> ::fnmock::expectation_handle::ExpectationHandle<GetUserMatcher> {
                        ::fnmock::expectation_handle::ExpectationHandle::new(
                            matcher,
                            "get_user",
                            |expectation| {
                                SPY.with_borrow_mut(|spy| {
                                    spy.add_expectation(expectation);
                                })
                            },
                            |sequences| {
                                SPY.with_borrow_mut(|spy| {
                                    spy.add_sequences(sequences);
                                })
                            },
                        )
                    }
                }
            }
        };

        assert_eq!(
            generated.to_token_stream().to_string(),
            expected.to_token_stream().to_string(),
        );
    }

    #[test]
    fn test_generate_spy_module_code_for_one_param() {
        let info = SpyModuleInfo {
            module_name: syn::parse_str("save_user_spy_module").unwrap(),
            store_name: syn::parse_str("SPY").unwrap(),
            display_name: "save_user".to_string(),
            matcher_name: syn::parse_str("SaveUserMatcher").unwrap(),
            param_idents: vec![syn::parse_str("id").unwrap()],
            param_types_unreferenced: vec![syn::parse_str("String").unwrap()],
            interface_struct_name: syn::parse_str("SaveUserSpyInterface").unwrap(),
        };

        let generated = generate_spy_module_code(&info).expect("generated module should parse");

        let expected: syn::ItemMod = syn::parse_quote! {
            #[cfg(test)]
            pub(crate) mod save_user_spy_module {
                use super::*;

                thread_local! {
                    static SPY: ::std::cell::RefCell<::fnmock::spy_store::SpyStore<SaveUserMatcher>> =
                        ::std::cell::RefCell::new(
                            ::fnmock::spy_store::SpyStore::new("save_user")
                        );
                }

                #[derive(Clone)]
                pub enum SaveUserMatcher {
                    Predicates {
                        id: ::std::rc::Rc<dyn ::fnmock::Predicate<String>>,
                    },
                    Function {
                        function: ::std::rc::Rc<dyn Fn(&String) -> bool>,
                    },
                }

                impl ::fnmock::matcher::Matcher for SaveUserMatcher {
                    type Params<'a> = (&'a String,);

                    fn matches(&self, params: &Self::Params<'_>) -> bool {
                        match self {
                            Self::Predicates { id } => id.eval(params.0),
                            Self::Function { function } => function(params.0),
                        }
                    }
                }

                impl ::std::fmt::Display for SaveUserMatcher {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        match self {
                            Self::Predicates { id } => {
                                write!(f, "{}", id.to_string().replacen("var", "id", 1))
                            },
                            Self::Function { .. } => {
                                write!(f, "a function predicate")
                            }
                        }
                    }
                }

                pub(super) fn internal_record_call(params: &(&String,)) {
                    SPY.with_borrow_mut(|spy| {
                        spy.record_call(params);
                    })
                }

                pub(super) fn internal_get_interface() -> SaveUserSpyInterface {
                    SaveUserSpyInterface {}
                }

                pub struct SaveUserSpyInterface {}

                impl SaveUserSpyInterface {
                    /// Expect calls whose arguments satisfy one predicate per parameter.
                    pub fn expect(
                        &self,
                        id: impl ::fnmock::Predicate<String> + 'static,
                    ) -> ::fnmock::expectation_handle::ExpectationHandle<SaveUserMatcher> {
                        self.set_expectation(SaveUserMatcher::Predicates {
                            id: ::std::rc::Rc::new(id),
                        })
                    }

                    /// Expect calls whose arguments satisfy `function`.
                    pub fn expectf(
                        &self,
                        function: impl Fn(&String) -> bool + 'static,
                    ) -> ::fnmock::expectation_handle::ExpectationHandle<SaveUserMatcher> {
                        self.set_expectation(SaveUserMatcher::Function {
                            function: ::std::rc::Rc::new(function),
                        })
                    }

                    /// Expect this many calls, whatever their arguments.
                    pub fn expect_times(&self, call_range: impl Into<::fnmock::call_range::CallRange>) {
                        SPY.with_borrow_mut(|spy| spy.set_total_call_range(call_range.into()));
                    }

                    /// Expect exactly one call, whatever its arguments.
                    pub fn expect_once(&self) {
                        self.expect_times(1);
                    }

                    /// Expect the function not to be called at all.
                    pub fn expect_never(&self) {
                        self.expect_times(0);
                    }

                    /// Record a call. Called by the spied function, not by the test.
                    pub fn record_call(&self, params: &(&String,)) {
                        SPY.with_borrow_mut(|spy| spy.record_call(params));
                    }

                    /// Assert every expectation set on this spy is fulfilled.
                    pub fn assert(&self) {
                        SPY.with_borrow(|spy| spy.assert());
                    }

                    fn set_expectation(
                        &self,
                        matcher: SaveUserMatcher,
                    ) -> ::fnmock::expectation_handle::ExpectationHandle<SaveUserMatcher> {
                        ::fnmock::expectation_handle::ExpectationHandle::new(
                            matcher,
                            "save_user",
                            |expectation| {
                                SPY.with_borrow_mut(|spy| {
                                    spy.add_expectation(expectation);
                                })
                            },
                            |sequences| {
                                SPY.with_borrow_mut(|spy| {
                                    spy.add_sequences(sequences);
                                })
                            },
                        )
                    }
                }
            }
        };

        assert_eq!(
            generated.to_token_stream().to_string(),
            expected.to_token_stream().to_string(),
        );
    }

    /// A zero-parameter function's params tuple must be the unit type, and its matcher's
    /// `Predicates` variant always matches since there is nothing to evaluate.
    #[test]
    fn test_generate_spy_module_code_for_zero_params() {
        let info = SpyModuleInfo {
            module_name: syn::parse_str("ping_spy_module").unwrap(),
            store_name: syn::parse_str("SPY").unwrap(),
            display_name: "ping".to_string(),
            matcher_name: syn::parse_str("PingMatcher").unwrap(),
            param_idents: vec![],
            param_types_unreferenced: vec![],
            interface_struct_name: syn::parse_str("PingSpyInterface").unwrap(),
        };

        let generated = generate_spy_module_code(&info).expect("generated module should parse");

        let expected: syn::ItemMod = syn::parse_quote! {
            #[cfg(test)]
            pub(crate) mod ping_spy_module {
                use super::*;

                thread_local! {
                    static SPY: ::std::cell::RefCell<::fnmock::spy_store::SpyStore<PingMatcher>> =
                        ::std::cell::RefCell::new(
                            ::fnmock::spy_store::SpyStore::new("ping")
                        );
                }

                #[derive(Clone)]
                pub enum PingMatcher {
                    Predicates {},
                    Function {
                        function: ::std::rc::Rc<dyn Fn() -> bool>,
                    },
                }

                impl ::fnmock::matcher::Matcher for PingMatcher {
                    type Params<'a> = ();

                    fn matches(&self, params: &Self::Params<'_>) -> bool {
                        match self {
                            Self::Predicates {} => true,
                            Self::Function { function } => function(),
                        }
                    }
                }

                impl ::std::fmt::Display for PingMatcher {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        match self {
                            Self::Predicates {} => {
                                write!(f, "",)
                            },
                            Self::Function { .. } => {
                                write!(f, "a function predicate")
                            }
                        }
                    }
                }

                pub(super) fn internal_record_call(params: &()) {
                    SPY.with_borrow_mut(|spy| {
                        spy.record_call(params);
                    })
                }

                pub(super) fn internal_get_interface() -> PingSpyInterface {
                    PingSpyInterface {}
                }

                pub struct PingSpyInterface {}

                impl PingSpyInterface {
                    /// Expect calls whose arguments satisfy one predicate per parameter.
                    pub fn expect(
                        &self,
                    ) -> ::fnmock::expectation_handle::ExpectationHandle<PingMatcher> {
                        self.set_expectation(PingMatcher::Predicates {})
                    }

                    /// Expect calls whose arguments satisfy `function`.
                    pub fn expectf(
                        &self,
                        function: impl Fn() -> bool + 'static,
                    ) -> ::fnmock::expectation_handle::ExpectationHandle<PingMatcher> {
                        self.set_expectation(PingMatcher::Function {
                            function: ::std::rc::Rc::new(function),
                        })
                    }

                    /// Expect this many calls, whatever their arguments.
                    pub fn expect_times(&self, call_range: impl Into<::fnmock::call_range::CallRange>) {
                        SPY.with_borrow_mut(|spy| spy.set_total_call_range(call_range.into()));
                    }

                    /// Expect exactly one call, whatever its arguments.
                    pub fn expect_once(&self) {
                        self.expect_times(1);
                    }

                    /// Expect the function not to be called at all.
                    pub fn expect_never(&self) {
                        self.expect_times(0);
                    }

                    /// Record a call. Called by the spied function, not by the test.
                    pub fn record_call(&self, params: &()) {
                        SPY.with_borrow_mut(|spy| spy.record_call(params));
                    }

                    /// Assert every expectation set on this spy is fulfilled.
                    pub fn assert(&self) {
                        SPY.with_borrow(|spy| spy.assert());
                    }

                    fn set_expectation(
                        &self,
                        matcher: PingMatcher,
                    ) -> ::fnmock::expectation_handle::ExpectationHandle<PingMatcher> {
                        ::fnmock::expectation_handle::ExpectationHandle::new(
                            matcher,
                            "ping",
                            |expectation| {
                                SPY.with_borrow_mut(|spy| {
                                    spy.add_expectation(expectation);
                                })
                            },
                            |sequences| {
                                SPY.with_borrow_mut(|spy| {
                                    spy.add_sequences(sequences);
                                })
                            },
                        )
                    }
                }
            }
        };

        assert_eq!(
            generated.to_token_stream().to_string(),
            expected.to_token_stream().to_string(),
        );
    }
}
