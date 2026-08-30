use quote::quote;

use crate::scheme::common::generic_scheme::GenericScheme;

/// Builds the matcher enum for a spied function, plus its `Matcher` and `Display` impls.
///
/// The matcher has one variant per way a test can set an expectation: `Predicates`, one boxed
/// [`Predicate`](fnmock::Predicate) per parameter (built by `expect`), and `Function`, a single
/// closure over every parameter at once (built by `expectf`). When `supports_expect` is `false`
/// the `Predicates` variant is left off entirely — see its doc comment on
/// [`FunctionSpyScheme`](crate::scheme::function::spy::FunctionSpyScheme) for why a parameter
/// can force that.
///
/// When `generic_scheme` is `Some`, the enum and both impls repeat the function's generic
/// parameters, and each variant carries an extra `_marker` field so every generic parameter is
/// used even when a variant's other fields don't happen to mention it (e.g. `expectf` when the
/// function has no parameters, or a generic that appears only in the return type). Building the
/// matching `Predicates { .. }`/`Function { .. }` construct expressions for that field is
/// [`build_marker_construct`]'s job, since it is needed wherever the interface builds a matcher
/// value, not just here.
///
/// `Matcher::Params<'a>` is set to `#params_name`, a tuple struct generated here rather than the
/// raw params tuple directly, so that a spied function's parameter types never have to be `pub`
/// themselves. `Matcher` is a trait `fnmock` declares `pub`, so naming a private type directly in
/// one of its impls' associated types is a hard privacy error (`E0446`) — but a private *field*
/// on an otherwise-`pub` wrapper struct is invisible to that check, the same way an ordinary
/// `pub fn foo() -> Wrapper` may freely return a `Wrapper` whose fields aren't `pub`. Its fields
/// are positional (`params.0`, `params.1`, ...) so [`matches`](Matcher::matches) can index into
/// it exactly like the raw tuple it replaces; a generic matcher appends the same kind of
/// `PhantomData` marker field as the enum, just unnamed to fit the tuple-struct shape.
pub fn build_matcher(
    matcher_name: &syn::Ident,
    params_name: &syn::Ident,
    param_idents: &[syn::Ident],
    param_types: &[syn::Type],
    params_tuple_types: &[syn::Type],
    generic_scheme: Option<&GenericScheme>,
    supports_expect: bool,
) -> proc_macro2::TokenStream {
    let marker_field = build_marker_field(generic_scheme);
    let has_marker = generic_scheme.is_some();

    // A struct pattern's fields, comma-joined, with `..` appended only for a generic matcher.
    // Built as a list rather than interpolating a trailing `#ignore_marker` token so that an
    // empty field list (zero params, non-generic) still renders as `{}` rather than `{ , }`.
    let pattern_fields = |idents: &[syn::Ident]| -> Vec<proc_macro2::TokenStream> {
        let mut fields: Vec<proc_macro2::TokenStream> =
            idents.iter().map(|i| quote! { #i }).collect();
        if has_marker {
            fields.push(quote! { .. });
        }
        fields
    };
    let predicates_pattern_fields = pattern_fields(param_idents);
    let function_pattern_fields = pattern_fields(std::slice::from_ref(&function_field_ident()));

    // Like `pattern_fields`, but binds `_marker` by name instead of ignoring it with `..`, for
    // the hand-written `Clone` impl below, which needs to clone that field too.
    let pattern_fields_binding_marker = |idents: &[syn::Ident]| -> Vec<proc_macro2::TokenStream> {
        let mut fields: Vec<proc_macro2::TokenStream> =
            idents.iter().map(|i| quote! { #i }).collect();
        if has_marker {
            fields.push(quote! { _marker });
        }
        fields
    };

    let predicates_variant = supports_expect.then(|| {
        let predicates_fields = param_idents.iter().zip(param_types).map(|(ident, ty)| {
            quote! { #ident: ::std::rc::Rc<dyn ::fnmock::Predicate<#ty>>, }
        });
        quote! {
            Predicates {
                #(#predicates_fields)*
                #marker_field
            },
        }
    });
    let function_signature = quote! { Fn(#(&#param_types),*) -> bool };

    let params_fields = params_tuple_types.iter().map(|ty| quote! { &'a #ty, });
    let params_generics_decl = build_params_generics_decl(generic_scheme);
    let params_generics_use = build_params_generics_use(generic_scheme);
    let params_marker_field =
        build_params_marker_field(generic_scheme, !params_tuple_types.is_empty());

    let indices = (0..param_idents.len()).map(syn::Index::from);
    let predicates_matches_expr = if param_idents.is_empty() {
        quote! { true }
    } else {
        let evals = param_idents
            .iter()
            .zip(indices.clone())
            .map(|(ident, index)| quote! { #ident.eval(params.#index) });
        quote! { #(#evals)&&* }
    };
    let function_matches_args = indices.map(|index| quote! { params.#index });
    let predicates_matches_arm = supports_expect.then(|| {
        quote! {
            Self::Predicates { #(#predicates_pattern_fields),* } => #predicates_matches_expr,
        }
    });

    let format_str = vec!["{}"; param_idents.len()].join(" && ");
    let display_args = param_idents.iter().map(|ident| {
        let name = ident.to_string();
        quote! { #ident.to_string().replacen("var", #name, 1) }
    });
    let predicates_display_arm = supports_expect.then(|| {
        quote! {
            Self::Predicates { #(#predicates_pattern_fields),* } => {
                write!(f, #format_str, #(#display_args),*)
            },
        }
    });

    let generics_decl = build_generics_decl(generic_scheme);
    let generics_use = build_generics_use(generic_scheme);

    // `#[derive(Clone)]` on a generic enum adds a `T: Clone` bound to the generated impl, even
    // though every field here (`Rc<..>`, `PhantomData<..>`) is `Clone` regardless of `T`. That
    // spurious bound would then be demanded of every spied generic function, since `Matcher`
    // requires `Clone`. A non-generic matcher has no such problem, so it keeps the derive.
    let (derive_clone, clone_impl) = if has_marker {
        let function_clone_pattern =
            pattern_fields_binding_marker(std::slice::from_ref(&function_field_ident()));
        let function_clone_fields =
            quote! { function: function.clone(), _marker: _marker.clone(), };
        let predicates_clone_arm = supports_expect.then(|| {
            let predicates_clone_pattern = pattern_fields_binding_marker(param_idents);
            let predicates_clone_fields = param_idents
                .iter()
                .map(|ident| quote! { #ident: #ident.clone(), })
                .chain(std::iter::once(quote! { _marker: _marker.clone(), }));
            quote! {
                Self::Predicates { #(#predicates_clone_pattern),* } => Self::Predicates {
                    #(#predicates_clone_fields)*
                },
            }
        });

        (
            quote! {},
            quote! {
                impl #generics_decl ::std::clone::Clone for #matcher_name #generics_use {
                    fn clone(&self) -> Self {
                        match self {
                            #predicates_clone_arm
                            Self::Function { #(#function_clone_pattern),* } => Self::Function {
                                #function_clone_fields
                            },
                        }
                    }
                }
            },
        )
    } else {
        (quote! { #[derive(Clone)] }, quote! {})
    };

    quote! {
        #derive_clone
        pub enum #matcher_name #generics_decl {
            #predicates_variant
            Function {
                function: ::std::rc::Rc<dyn #function_signature>,
                #marker_field
            },
        }

        #clone_impl

        pub struct #params_name #params_generics_decl (
            #(#params_fields)*
            #params_marker_field
        );

        impl #generics_decl ::fnmock::matcher::Matcher for #matcher_name #generics_use {
            type Params<'a> = #params_name #params_generics_use;

            fn matches(&self, params: &Self::Params<'_>) -> bool {
                match self {
                    #predicates_matches_arm
                    Self::Function { #(#function_pattern_fields),* } => function(#(#function_matches_args),*),
                }
            }
        }

        impl #generics_decl ::std::fmt::Display for #matcher_name #generics_use {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    #predicates_display_arm
                    Self::Function { .. } => {
                        write!(f, "a function predicate")
                    }
                }
            }
        }
    }
}

fn function_field_ident() -> syn::Ident {
    syn::Ident::new("function", proc_macro2::Span::call_site())
}

/// Builds `<T: 'static>` (or empty) for declaring generic parameters, matching how the rest of
/// the codebase's builders only emit angle brackets when there is something to put in them.
fn build_generics_decl(generic_scheme: Option<&GenericScheme>) -> proc_macro2::TokenStream {
    let Some(generic_scheme) = generic_scheme else {
        return quote! {};
    };
    let params = &generic_scheme.params;
    quote! { <#(#params),*> }
}

/// Builds `<T>` (or empty) for using previously-declared generic parameters.
fn build_generics_use(generic_scheme: Option<&GenericScheme>) -> proc_macro2::TokenStream {
    let Some(generic_scheme) = generic_scheme else {
        return quote! {};
    };
    let idents = &generic_scheme.idents;
    quote! { <#(#idents),*> }
}

/// Builds the `_marker` field declaration for the matcher enum's variants, empty when there is
/// no generic scheme.
fn build_marker_field(generic_scheme: Option<&GenericScheme>) -> proc_macro2::TokenStream {
    let Some(generic_scheme) = generic_scheme else {
        return quote! {};
    };
    let idents = &generic_scheme.idents_without_const_generics;
    quote! { _marker: ::std::marker::PhantomData<(#(#idents),*)>, }
}

/// Builds the `_marker: ..` field the interface has to add when it constructs a `Predicates` or
/// `Function` matcher value, empty when there is no generic scheme.
pub fn build_marker_construct(generic_scheme: Option<&GenericScheme>) -> proc_macro2::TokenStream {
    if generic_scheme.is_some() {
        quote! { _marker: ::std::marker::PhantomData, }
    } else {
        quote! {}
    }
}

/// Builds the params wrapper struct's own generic declaration, e.g. `<'a>` or `<'a, T: 'static>`:
/// like [`build_generics_decl`], but the wrapper always has the `'a` from `Matcher::Params<'a>`
/// in addition to the function's own generics, so it needs angle brackets even when the function
/// isn't generic.
fn build_params_generics_decl(generic_scheme: Option<&GenericScheme>) -> proc_macro2::TokenStream {
    let Some(generic_scheme) = generic_scheme else {
        return quote! { <'a> };
    };
    let params = &generic_scheme.params;
    quote! { <'a, #(#params),*> }
}

/// Builds the params wrapper struct's own generic use, e.g. `<'a>` or `<'a, T>`; see
/// [`build_params_generics_decl`].
fn build_params_generics_use(generic_scheme: Option<&GenericScheme>) -> proc_macro2::TokenStream {
    let Some(generic_scheme) = generic_scheme else {
        return quote! { <'a> };
    };
    let idents = &generic_scheme.idents;
    quote! { <'a, #(#idents),*> }
}

/// Whether the params wrapper struct needs a trailing `PhantomData` field/value at all: either
/// because the spied function takes no parameters, so there are no other fields left to use the
/// wrapper's `'a` (a hard error, `E0392`, independent of whether the function is generic), or
/// because some non-const generic parameter isn't otherwise mentioned by a parameter type (e.g. a
/// generic that appears only in the return type). Const generics don't count: unlike type
/// parameters, an unused const generic on a struct isn't an error.
///
/// [`build_params_marker_field`] and [`build_params_marker_construct`] both have to agree on this
/// exactly, since one declares the field and the other constructs a value for it — sharing this
/// check is what keeps them from drifting apart.
fn params_needs_marker(generic_scheme: Option<&GenericScheme>, has_params: bool) -> bool {
    if !has_params {
        return true;
    }
    generic_scheme.is_some_and(|g| !g.idents_without_const_generics.is_empty())
}

/// Builds the params wrapper struct's trailing `PhantomData<(..)>,` field, empty when
/// [`params_needs_marker`] says it isn't needed: like [`build_marker_field`], but unnamed to fit
/// the wrapper's tuple-struct shape (its other fields are positional, indexed by
/// [`matches`](Matcher::matches) as `params.0`, `params.1`, ...). Folds a `&'a ()` into the same
/// `PhantomData` that already carries any unused generic parameters, when the function takes no
/// parameters to use `'a` through some other field.
fn build_params_marker_field(
    generic_scheme: Option<&GenericScheme>,
    has_params: bool,
) -> proc_macro2::TokenStream {
    if !params_needs_marker(generic_scheme, has_params) {
        return quote! {};
    }
    let generic_idents = generic_scheme
        .map(|g| g.idents_without_const_generics.as_slice())
        .unwrap_or_default();
    let lifetime_marker = (!has_params).then(|| quote! { &'a (), });
    quote! { ::std::marker::PhantomData<(#lifetime_marker #(#generic_idents),*)>, }
}

/// Builds the trailing `PhantomData,` value `internal_record_call` has to add when it constructs
/// a params wrapper value, empty when [`params_needs_marker`] says it isn't needed; the unnamed
/// counterpart of [`build_marker_construct`], for the same reason [`build_params_marker_field`]
/// is the unnamed counterpart of [`build_marker_field`].
pub fn build_params_marker_construct(
    generic_scheme: Option<&GenericScheme>,
    has_params: bool,
) -> proc_macro2::TokenStream {
    if params_needs_marker(generic_scheme, has_params) {
        quote! { ::std::marker::PhantomData, }
    } else {
        quote! {}
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    fn non_generic_matcher() -> proc_macro2::TokenStream {
        let matcher_name: syn::Ident = parse_quote!(GetUserMatcher);
        let params_name: syn::Ident = parse_quote!(GetUserMatcherParams);
        let param_idents: Vec<syn::Ident> = vec![parse_quote!(id), parse_quote!(uuid)];
        let param_types: Vec<syn::Type> = vec![parse_quote!(String), parse_quote!(str)];

        build_matcher(
            &matcher_name,
            &params_name,
            &param_idents,
            &param_types,
            &param_types,
            None,
            true,
        )
    }

    #[test]
    fn test_non_generic_multiple_params() {
        let res = non_generic_matcher();

        let expected = quote! {
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

            pub struct GetUserMatcherParams<'a>(
                &'a String,
                &'a str,
            );

            impl ::fnmock::matcher::Matcher for GetUserMatcher {
                type Params<'a> = GetUserMatcherParams<'a>;

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
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_non_generic_zero_params() {
        let matcher_name: syn::Ident = parse_quote!(PingMatcher);
        let params_name: syn::Ident = parse_quote!(PingMatcherParams);

        let res = build_matcher(&matcher_name, &params_name, &[], &[], &[], None, true);

        let expected = quote! {
            #[derive(Clone)]
            pub enum PingMatcher {
                Predicates {},
                Function {
                    function: ::std::rc::Rc<dyn Fn() -> bool>,
                },
            }

            pub struct PingMatcherParams<'a>(
                ::std::marker::PhantomData<(&'a (),)>,
            );

            impl ::fnmock::matcher::Matcher for PingMatcher {
                type Params<'a> = PingMatcherParams<'a>;

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
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_generic_matcher_adds_marker_field_and_repeats_generics() {
        let matcher_name: syn::Ident = parse_quote!(FooMatcher);
        let params_name: syn::Ident = parse_quote!(FooMatcherParams);
        let param_idents: Vec<syn::Ident> = vec![parse_quote!(a)];
        let param_types: Vec<syn::Type> = vec![parse_quote!(T)];
        let generic_scheme = GenericScheme {
            params: vec![parse_quote!(T: 'static)],
            idents: vec![parse_quote!(T)],
            idents_without_const_generics: vec![parse_quote!(T)],
            keys: vec![parse_quote!(::std::any::TypeId::of::<T>())],
        };

        let res = build_matcher(
            &matcher_name,
            &params_name,
            &param_idents,
            &param_types,
            &param_types,
            Some(&generic_scheme),
            true,
        );

        let expected = quote! {
            pub enum FooMatcher<T: 'static> {
                Predicates {
                    a: ::std::rc::Rc<dyn ::fnmock::Predicate<T>>,
                    _marker: ::std::marker::PhantomData<(T)>,
                },
                Function {
                    function: ::std::rc::Rc<dyn Fn(&T) -> bool>,
                    _marker: ::std::marker::PhantomData<(T)>,
                },
            }

            impl<T: 'static> ::std::clone::Clone for FooMatcher<T> {
                fn clone(&self) -> Self {
                    match self {
                        Self::Predicates { a, _marker } => Self::Predicates {
                            a: a.clone(),
                            _marker: _marker.clone(),
                        },
                        Self::Function { function, _marker } => Self::Function {
                            function: function.clone(),
                            _marker: _marker.clone(),
                        },
                    }
                }
            }

            pub struct FooMatcherParams<'a, T: 'static>(
                &'a T,
                ::std::marker::PhantomData<(T)>,
            );

            impl<T: 'static> ::fnmock::matcher::Matcher for FooMatcher<T> {
                type Params<'a> = FooMatcherParams<'a, T>;

                fn matches(&self, params: &Self::Params<'_>) -> bool {
                    match self {
                        Self::Predicates { a, .. } => a.eval(params.0),
                        Self::Function { function, .. } => function(params.0),
                    }
                }
            }

            impl<T: 'static> ::std::fmt::Display for FooMatcher<T> {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::Predicates { a, .. } => {
                            write!(f, "{}", a.to_string().replacen("var", "a", 1))
                        },
                        Self::Function { .. } => {
                            write!(f, "a function predicate")
                        }
                    }
                }
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_supports_expect_false_omits_predicates_variant_entirely() {
        let matcher_name: syn::Ident = parse_quote!(LifetimeParamTypeMatcher);
        let params_name: syn::Ident = parse_quote!(LifetimeParamTypeMatcherParams);
        let param_idents: Vec<syn::Ident> = vec![parse_quote!(r)];
        let param_types: Vec<syn::Type> = vec![parse_quote!(Ref<>)];
        let params_tuple_types: Vec<syn::Type> = vec![parse_quote!(Ref<'a>)];

        let res = build_matcher(
            &matcher_name,
            &params_name,
            &param_idents,
            &param_types,
            &params_tuple_types,
            None,
            false,
        );

        let expected = quote! {
            #[derive(Clone)]
            pub enum LifetimeParamTypeMatcher {
                Function {
                    function: ::std::rc::Rc<dyn Fn(&Ref<>) -> bool>,
                },
            }

            pub struct LifetimeParamTypeMatcherParams<'a>(
                &'a Ref<'a>,
            );

            impl ::fnmock::matcher::Matcher for LifetimeParamTypeMatcher {
                type Params<'a> = LifetimeParamTypeMatcherParams<'a>;

                fn matches(&self, params: &Self::Params<'_>) -> bool {
                    match self {
                        Self::Function { function } => function(params.0),
                    }
                }
            }

            impl ::std::fmt::Display for LifetimeParamTypeMatcher {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::Function { .. } => {
                            write!(f, "a function predicate")
                        }
                    }
                }
            }
        };

        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn test_build_marker_construct_non_generic_is_empty() {
        assert_eq!(build_marker_construct(None).to_string(), "");
    }

    #[test]
    fn test_build_marker_construct_generic() {
        let generic_scheme = GenericScheme {
            params: vec![parse_quote!(T)],
            idents: vec![parse_quote!(T)],
            idents_without_const_generics: vec![parse_quote!(T)],
            keys: vec![],
        };

        assert_eq!(
            build_marker_construct(Some(&generic_scheme)).to_string(),
            quote! { _marker: ::std::marker::PhantomData, }.to_string()
        );
    }
}
