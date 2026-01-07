use quote::quote;
use crate::use_tree_processor::process_use_tree;

/// Processes a use statement and generates conditional imports for modified versions.
///
/// This is a shared implementation that can be used for both mocks and fakes.
/// It analyzes a use statement and generates conditional compilation attributes that:
/// - Import the original functions in production builds
/// - Import modified versions (with custom suffix) aliased to original names in test builds
///
/// # Arguments
///
/// * `input` - The use statement to process
/// * `suffix` - The suffix to append to function names (e.g., "_mock" or "_fake")
///
/// # Returns
///
/// - `Ok(TokenStream2)` - The expanded code with conditional imports
/// - `Err(syn::Error)` - If the use statement cannot be processed
pub(crate) fn process_use_statement(
    input: syn::ItemUse,
    suffix: &str,
) -> syn::Result<proc_macro2::TokenStream> {
    // Extract the module path and function name mappings
    let mut base_path = Vec::new();
    let function_mappings = process_use_tree(&input.tree, &mut base_path);

    // Generate modified function names with the suffix
    let modified_mappings: Vec<_> = function_mappings
        .iter()
        .map(|(fn_name, _)| {
            let modified_fn_name = syn::Ident::new(
                &format!("{}{}", fn_name, suffix),
                fn_name.span(),
            );
            (fn_name.clone(), modified_fn_name)
        })
        .collect();

    // Reconstruct the module path as tokens
    let module_path = if base_path.is_empty() {
        quote! {}
    } else {
        quote! { #(#base_path)::* }
    };

    Ok(
        // Generate the appropriate expansion based on number of imports
        if modified_mappings.len() == 1 {
            let (fn_name, modified_fn_name) = &modified_mappings[0];
            generate_single_import(&input, module_path, fn_name, modified_fn_name)
        } else {
            generate_multiple_imports(&input, module_path, &modified_mappings)
        }
    )
}

/// Generates the expanded code for a single function import with modified version.
///
/// Creates conditional compilation attributes that import the original function
/// in production builds and the modified version (aliased to the original name) in test builds.
///
/// # Arguments
///
/// * `input` - The original use statement
/// * `module_path` - The module path tokens (empty if importing from current module)
/// * `fn_name` - The original function name
/// * `modified_fn_name` - The modified function name (with suffix)
///
/// # Returns
///
/// Token stream containing:
/// ```ignore
/// #[cfg(not(test))]
/// use original::statement;
/// #[cfg(test)]
/// use module::path::function_modified as function;
/// ```
fn generate_single_import(
    input: &syn::ItemUse,
    module_path: proc_macro2::TokenStream,
    fn_name: &syn::Ident,
    modified_fn_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        #[cfg(not(test))]
        #input
        
        #[cfg(test)]
        use #module_path::#modified_fn_name as #fn_name;
    }
}

/// Generates the expanded code for multiple function imports with modified versions.
///
/// Creates conditional compilation attributes that import the original functions
/// in production builds and the modified versions (aliased to the original names) in test builds.
///
/// # Arguments
///
/// * `input` - The original use statement
/// * `module_path` - The module path tokens (empty if importing from current module)
/// * `function_mappings` - Vector of (original_name, modified_name) tuples
///
/// # Returns
///
/// Token stream containing:
/// ```ignore
/// #[cfg(not(test))]
/// use original::statement;
/// #[cfg(test)]
/// use module::path::{fn1_modified as fn1, fn2_modified as fn2};
/// ```
fn generate_multiple_imports(
    input: &syn::ItemUse,
    module_path: proc_macro2::TokenStream,
    function_mappings: &[(syn::Ident, syn::Ident)],
) -> proc_macro2::TokenStream {
    let alias_mappings: Vec<_> = function_mappings
        .iter()
        .map(|(fn_name, modified_fn_name)| {
            quote! { #modified_fn_name as #fn_name }
        })
        .collect();
    
    quote! {
        #[cfg(not(test))]
        #input
        
        #[cfg(test)]
        use #module_path::{#(#alias_mappings),*};
    }
}
