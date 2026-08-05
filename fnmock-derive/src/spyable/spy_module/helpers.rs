use quote::quote;

/// Builds a tuple type of references to `param_types_unreferenced`, one per parameter, each
/// prefixed with `lifetime` (e.g. `'a`, or nothing to elide it).
///
/// Always emits a trailing comma after every element, so single-parameter functions get a real
/// 1-tuple (`(&'a String,)`) rather than a parenthesized type, and zero-parameter functions get
/// the unit type (`()`).
pub fn build_param_reference_tuple_type(
    param_types_unreferenced: &[syn::Type],
    lifetime: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let refs = param_types_unreferenced
        .iter()
        .map(|ty| quote! { &#lifetime #ty, });

    quote! { (#(#refs)*) }
}

/// Builds `matcher_predicates_fields` — see the field's own doc comment.
pub fn build_matcher_predicates_fields(
    param_idents: &[syn::Ident],
    param_types_unreferenced: &[syn::Type],
) -> proc_macro2::TokenStream {
    let fields = param_idents
        .iter()
        .zip(param_types_unreferenced)
        .map(|(ident, ty)| {
            quote! {
                #ident: ::std::rc::Rc<dyn ::fnmock::Predicate<#ty>>,
            }
        });

    quote! { #(#fields)* }
}

/// Builds `matcher_function_signature` — see the field's own doc comment.
pub fn build_matcher_function_signature(
    param_types_unreferenced: &[syn::Type],
) -> proc_macro2::TokenStream {
    quote! {
        Fn(#(&#param_types_unreferenced),*) -> bool
    }
}

/// Builds `matcher_predicates_matches_arm` — see the field's own doc comment.
pub fn build_matcher_predicates_matches_arm(
    param_idents: &[syn::Ident],
) -> proc_macro2::TokenStream {
    let indices = (0..param_idents.len()).map(syn::Index::from);
    let evals: Vec<_> = param_idents
        .iter()
        .zip(indices)
        .map(|(ident, index)| quote! { #ident.eval(params.#index) })
        .collect();

    // A function with no parameters has nothing to evaluate, so it always matches.
    let expr = if evals.is_empty() {
        quote! { true }
    } else {
        quote! { #(#evals)&&* }
    };

    quote! {
        Self::Predicates { #(#param_idents),* } => #expr
    }
}

/// Builds `matcher_function_matches_arm` — see the field's own doc comment.
pub fn build_matcher_function_matches_arm(param_idents: &[syn::Ident]) -> proc_macro2::TokenStream {
    let indices = (0..param_idents.len()).map(syn::Index::from);
    let args = indices.map(|index| quote! { params.#index });

    quote! {
        Self::Function { function } => function(#(#args),*)
    }
}

/// Builds `matcher_predicates_display_arm` — see the field's own doc comment.
pub fn build_matcher_predicates_display_arm(
    param_idents: &[syn::Ident],
) -> proc_macro2::TokenStream {
    let format_str = vec!["{}"; param_idents.len()].join(" && ");
    let args = param_idents.iter().map(|ident| {
        let name = ident.to_string();
        quote! { #ident.to_string().replacen("var", #name, 1) }
    });

    quote! {
        Self::Predicates { #(#param_idents),* } => {
            write!(f, #format_str, #(#args),*)
        }
    }
}

/// Builds `expect_params` — see the field's own doc comment.
pub fn build_expect_params(
    param_idents: &[syn::Ident],
    param_types_unreferenced: &[syn::Type],
) -> proc_macro2::TokenStream {
    let params = param_idents
        .iter()
        .zip(param_types_unreferenced)
        .map(|(ident, ty)| {
            quote! {
                #ident: impl ::fnmock::Predicate<#ty> + 'static,
            }
        });

    quote! { #(#params)* }
}

/// Builds `expect_construct_fields` — see the field's own doc comment.
pub fn build_expect_construct_fields(param_idents: &[syn::Ident]) -> proc_macro2::TokenStream {
    let fields = param_idents.iter().map(|ident| {
        quote! {
            #ident: ::std::rc::Rc::new(#ident),
        }
    });

    quote! { #(#fields)* }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parses two `&str, str, ...` type lists and builds the `syn::Ident`/`syn::Type` vectors
    /// most of these tests need, so each test case only has to spell out the raw idents/types.
    fn idents(names: &[&str]) -> Vec<syn::Ident> {
        names
            .iter()
            .map(|name| syn::parse_str(name).expect("expected input to parse as a syn::Ident"))
            .collect()
    }

    fn types(reprs: &[&str]) -> Vec<syn::Type> {
        reprs
            .iter()
            .map(|repr| syn::parse_str(repr).expect("expected input to parse as a syn::Type"))
            .collect()
    }

    mod build_param_reference_tuple_type_tests {
        use super::*;

        #[test]
        fn zero_params_is_unit_type_regardless_of_lifetime() {
            let result = build_param_reference_tuple_type(&[], quote! { 'a });

            assert_eq!(result.to_string(), quote! { () }.to_string());
        }

        #[test]
        fn one_param_with_named_lifetime_is_a_real_1_tuple() {
            let param_types = types(&["String"]);

            let result = build_param_reference_tuple_type(&param_types, quote! { 'a });

            assert_eq!(
                result.to_string(),
                quote! { (&'a String,) }.to_string(),
                "a single parameter must still produce a trailing comma, not a parenthesized type"
            );
        }

        #[test]
        fn one_param_with_elided_lifetime_has_no_lifetime_token() {
            let param_types = types(&["String"]);

            let result = build_param_reference_tuple_type(&param_types, quote! {});

            assert_eq!(result.to_string(), quote! { (&String,) }.to_string());
        }

        #[test]
        fn multiple_params_with_named_lifetime() {
            let param_types = types(&["String", "str"]);

            let result = build_param_reference_tuple_type(&param_types, quote! { 'a });

            assert_eq!(
                result.to_string(),
                quote! { (&'a String, &'a str,) }.to_string()
            );
        }

        #[test]
        fn multiple_params_with_elided_lifetime() {
            let param_types = types(&["String", "str"]);

            let result = build_param_reference_tuple_type(&param_types, quote! {});

            assert_eq!(result.to_string(), quote! { (&String, &str,) }.to_string());
        }
    }

    mod build_matcher_predicates_fields_tests {
        use super::*;

        #[test]
        fn zero_params_is_empty() {
            let result = build_matcher_predicates_fields(&[], &[]);

            assert_eq!(result.to_string(), String::new());
        }

        #[test]
        fn one_param() {
            let param_idents = idents(&["id"]);
            let param_types = types(&["String"]);

            let result = build_matcher_predicates_fields(&param_idents, &param_types);

            assert_eq!(
                result.to_string(),
                quote! { id: ::std::rc::Rc<dyn ::fnmock::Predicate<String>>, }.to_string()
            );
        }

        #[test]
        fn multiple_params_are_emitted_in_order() {
            let param_idents = idents(&["id", "uuid"]);
            let param_types = types(&["String", "str"]);

            let result = build_matcher_predicates_fields(&param_idents, &param_types);

            assert_eq!(
                result.to_string(),
                quote! {
                    id: ::std::rc::Rc<dyn ::fnmock::Predicate<String>>,
                    uuid: ::std::rc::Rc<dyn ::fnmock::Predicate<str>>,
                }
                .to_string()
            );
        }
    }

    mod build_matcher_function_signature_tests {
        use super::*;

        #[test]
        fn zero_params_has_empty_arg_list() {
            let result = build_matcher_function_signature(&[]);

            assert_eq!(result.to_string(), quote! { Fn() -> bool }.to_string());
        }

        #[test]
        fn one_param_is_referenced() {
            let param_types = types(&["String"]);

            let result = build_matcher_function_signature(&param_types);

            assert_eq!(
                result.to_string(),
                quote! { Fn(&String) -> bool }.to_string()
            );
        }

        #[test]
        fn multiple_params_are_comma_separated_and_referenced() {
            let param_types = types(&["String", "str"]);

            let result = build_matcher_function_signature(&param_types);

            assert_eq!(
                result.to_string(),
                quote! { Fn(&String, &str) -> bool }.to_string()
            );
        }
    }

    mod build_matcher_predicates_matches_arm_tests {
        use super::*;

        /// With no parameters there is nothing to evaluate, so the arm must unconditionally
        /// match rather than emit e.g. an empty `&&` chain.
        #[test]
        fn zero_params_always_matches() {
            let result = build_matcher_predicates_matches_arm(&[]);

            assert_eq!(
                result.to_string(),
                quote! { Self::Predicates {} => true }.to_string()
            );
        }

        #[test]
        fn one_param_evaluates_against_tuple_index_zero() {
            let param_idents = idents(&["id"]);

            let result = build_matcher_predicates_matches_arm(&param_idents);

            assert_eq!(
                result.to_string(),
                quote! { Self::Predicates { id } => id.eval(params.0) }.to_string()
            );
        }

        #[test]
        fn multiple_params_are_and_chained_in_declaration_order() {
            let param_idents = idents(&["id", "uuid"]);

            let result = build_matcher_predicates_matches_arm(&param_idents);

            assert_eq!(
                result.to_string(),
                quote! {
                    Self::Predicates { id, uuid } => id.eval(params.0) && uuid.eval(params.1)
                }
                .to_string()
            );
        }
    }

    mod build_matcher_function_matches_arm_tests {
        use super::*;

        #[test]
        fn zero_params_calls_function_with_no_args() {
            let result = build_matcher_function_matches_arm(&[]);

            assert_eq!(
                result.to_string(),
                quote! { Self::Function { function } => function() }.to_string()
            );
        }

        #[test]
        fn one_param_calls_function_with_tuple_index_zero() {
            let param_idents = idents(&["id"]);

            let result = build_matcher_function_matches_arm(&param_idents);

            assert_eq!(
                result.to_string(),
                quote! { Self::Function { function } => function(params.0) }.to_string()
            );
        }

        #[test]
        fn multiple_params_call_function_with_all_tuple_indices_in_order() {
            let param_idents = idents(&["id", "uuid"]);

            let result = build_matcher_function_matches_arm(&param_idents);

            assert_eq!(
                result.to_string(),
                quote! {
                    Self::Function { function } => function(params.0, params.1)
                }
                .to_string()
            );
        }
    }

    mod build_matcher_predicates_display_arm_tests {
        use super::*;

        /// The zero-param format string is empty, but the template still emits the trailing
        /// comma from `write!(f, #format_str, #(#args),*)` since that comma is a literal token
        /// in the template, not part of the (empty) repetition.
        #[test]
        fn zero_params_has_empty_format_string() {
            let result = build_matcher_predicates_display_arm(&[]);

            assert_eq!(
                result.to_string(),
                quote! {
                    Self::Predicates {} => {
                        write!(f, "",)
                    }
                }
                .to_string()
            );
        }

        #[test]
        fn one_param_replaces_var_with_the_param_name() {
            let param_idents = idents(&["id"]);

            let result = build_matcher_predicates_display_arm(&param_idents);

            assert_eq!(
                result.to_string(),
                quote! {
                    Self::Predicates { id } => {
                        write!(f, "{}", id.to_string().replacen("var", "id", 1))
                    }
                }
                .to_string()
            );
        }

        #[test]
        fn multiple_params_join_the_format_string_with_and_and_replace_each_name() {
            let param_idents = idents(&["id", "uuid"]);

            let result = build_matcher_predicates_display_arm(&param_idents);

            assert_eq!(
                result.to_string(),
                quote! {
                    Self::Predicates { id, uuid } => {
                        write!(
                            f,
                            "{} && {}",
                            id.to_string().replacen("var", "id", 1),
                            uuid.to_string().replacen("var", "uuid", 1)
                        )
                    }
                }
                .to_string()
            );
        }

        /// `replacen`'s replacement count is fixed at 1, so only the first `"var"` occurrence in
        /// a predicate's `Display` output is replaced with the param name — this locks in that
        /// the literal `1` in the generated call doesn't silently change.
        #[test]
        fn only_replaces_the_first_var_occurrence() {
            let param_idents = idents(&["id"]);

            let result = build_matcher_predicates_display_arm(&param_idents);

            assert!(
                result
                    .to_string()
                    .contains(&quote! { .replacen("var", "id", 1) }.to_string()),
                "expected the replacen call to cap replacements at 1"
            );
        }
    }

    mod build_expect_params_tests {
        use super::*;

        #[test]
        fn zero_params_is_empty() {
            let result = build_expect_params(&[], &[]);

            assert_eq!(result.to_string(), String::new());
        }

        #[test]
        fn one_param() {
            let param_idents = idents(&["id"]);
            let param_types = types(&["String"]);

            let result = build_expect_params(&param_idents, &param_types);

            assert_eq!(
                result.to_string(),
                quote! { id: impl ::fnmock::Predicate<String> + 'static, }.to_string()
            );
        }

        #[test]
        fn multiple_params_are_emitted_in_order() {
            let param_idents = idents(&["id", "uuid"]);
            let param_types = types(&["String", "str"]);

            let result = build_expect_params(&param_idents, &param_types);

            assert_eq!(
                result.to_string(),
                quote! {
                    id: impl ::fnmock::Predicate<String> + 'static,
                    uuid: impl ::fnmock::Predicate<str> + 'static,
                }
                .to_string()
            );
        }
    }

    mod build_expect_construct_fields_tests {
        use super::*;

        #[test]
        fn zero_params_is_empty() {
            let result = build_expect_construct_fields(&[]);

            assert_eq!(result.to_string(), String::new());
        }

        #[test]
        fn one_param() {
            let param_idents = idents(&["id"]);

            let result = build_expect_construct_fields(&param_idents);

            assert_eq!(
                result.to_string(),
                quote! { id: ::std::rc::Rc::new(id), }.to_string()
            );
        }

        #[test]
        fn multiple_params_are_emitted_in_order() {
            let param_idents = idents(&["id", "uuid"]);

            let result = build_expect_construct_fields(&param_idents);

            assert_eq!(
                result.to_string(),
                quote! {
                    id: ::std::rc::Rc::new(id),
                    uuid: ::std::rc::Rc::new(uuid),
                }
                .to_string()
            );
        }
    }
}
