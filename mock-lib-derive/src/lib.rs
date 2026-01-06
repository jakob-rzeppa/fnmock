use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Type, ItemUse};
use syn::punctuated::Punctuated;
use syn::token::Comma;

// Helper function to convert snake_case to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .filter_map(|w| {
            let mut c = w.chars();
            c.next().map(|f| f.to_uppercase().chain(c).collect::<String>())
        })
        .collect()
}

fn create_param_type(fn_inputs: &Punctuated<FnArg, Comma>) -> Type {
    // Extract parameter types for function pointer
    let param_types: Vec<_> = fn_inputs.iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => Some(&pat_type.ty),
            _ => panic!("self parameters not supported"),
        })
        .collect();

    // Create params type (single type or tuple)
    if param_types.len() == 1 {
        param_types[0].as_ref().clone()
    } else {
        syn::parse2(quote! { (#(#param_types),*) }).unwrap()
    }
}

fn create_tuple_from_param_names(fn_inputs: &Punctuated<FnArg, Comma>) -> proc_macro2::TokenStream {
    // Extract parameter names
    let param_names: Vec<_> = fn_inputs.iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => Some(&pat_type.pat),
            _ => panic!("self parameters not supported"),
        })
        .collect();

    // Create tuple from parameter names
    if param_names.is_empty() {
        quote! { () }
    } else if param_names.len() == 1 {
        let name = &param_names[0];
        quote! { #name }
    } else {
        quote! { (#(#param_names),*) }
    }
}

#[proc_macro_attribute]
pub fn mock_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    // Extract function details
    let fn_visibility = input.vis.clone();
    let fn_name = input.sig.ident.clone();
    let fn_inputs = input.sig.inputs.clone();
    let fn_output = input.sig.output.clone();
    let fn_block = input.block.clone();

    // Generate mock function name
    let mock_fn_name = syn::Ident::new(&format!("{}_mock", &fn_name), fn_name.span());

    let params_type = create_param_type(&fn_inputs);
    let params_to_tuple = create_tuple_from_param_names(&fn_inputs);
    
    // Extract return type from ReturnType
    let return_type: Type = match &input.sig.output {
        syn::ReturnType::Default => syn::parse2(quote! { () }).unwrap(),
        syn::ReturnType::Type(_, ty) => (**ty).clone(),
    };

    // Generate both the original function and the mock module
    let expanded = quote! {
        #fn_visibility fn #fn_name(#fn_inputs) #fn_output #fn_block

        pub(crate) fn #mock_fn_name(#fn_inputs) #fn_output {
            #mock_fn_name::call(#params_to_tuple)
        }

        pub(crate) mod #mock_fn_name {
            type Params = #params_type;
            type Return = #return_type;
            const FUNCTION_NAME: &str = "#mock_fn_name";

            thread_local! {
                static MOCK: std::cell::RefCell<mock_lib::function_mock::FunctionMock<
                    Params,
                    Return,
                >> = std::cell::RefCell::new(mock_lib::function_mock::FunctionMock::new(FUNCTION_NAME));
            }

            pub(crate) fn call(params: Params) -> Return {
                MOCK.with(|mock| {
                    mock.borrow_mut().call(params)
                })
            }

            pub(crate) fn mock_implementation(new_f: fn(Params) -> Return) {
                MOCK.with(|mock| {
                    mock.borrow_mut().mock_implementation(new_f)
                })
            }

            pub(crate) fn clear_mock() {
                MOCK.with(|mock|{
                    mock.borrow_mut().clear_mock()
                })
            }

            pub(crate) fn assert_times(expected_num_of_calls: u32) {
                MOCK.with(|mock| {
                    mock.borrow().assert_times(expected_num_of_calls)
                })
            }

            pub(crate) fn assert_with(params: Params) {
                MOCK.with(|mock| {
                    mock.borrow().assert_with(params)
                })
            }
        }
    };

    TokenStream::from(expanded)
}

fn process_use_tree(
    tree: &syn::UseTree,
    base_path: &mut Vec<syn::Ident>,
) -> Vec<(syn::Ident, syn::Ident)> {
    match tree {
        syn::UseTree::Path(path) => {
            base_path.push(path.ident.clone());
            process_use_tree(&path.tree, base_path)
        }
        syn::UseTree::Name(name) => {
            let fn_name = name.ident.clone();
            let mock_fn_name = syn::Ident::new(
                &format!("{}_mock", fn_name),
                fn_name.span()
            );
            vec![(fn_name, mock_fn_name)]
        }
        syn::UseTree::Group(group) => {
            let mut results = Vec::new();
            for item in &group.items {
                let mut path = base_path.clone();
                results.extend(process_use_tree(item, &mut path));
            }
            results
        }
        _ => panic!("Unsupported use tree format"),
    }
}

#[proc_macro_attribute]
pub fn use_function_mock(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemUse);
    
    let mut base_path = Vec::new();
    let mappings = process_use_tree(&input.tree, &mut base_path);
    
    // Reconstruct the base path
    let base_path_tokens = if base_path.is_empty() {
        quote! {}
    } else {
        quote! { #(#base_path)::* }
    };
    
    if mappings.len() == 1 {
        // Single import: use path::function;
        let (fn_name, mock_fn_name) = &mappings[0];
        let expanded = quote! {
            #[cfg(not(test))]
            #input
            
            #[cfg(test)]
            use #base_path_tokens::#mock_fn_name as #fn_name;
        };
        TokenStream::from(expanded)
    } else {
        // Multiple imports: use path::{fn1, fn2};
        let fn_names: Vec<_> = mappings.iter().map(|(fn_name, _)| fn_name).collect();
        let mock_mappings: Vec<_> = mappings.iter().map(|(fn_name, mock_fn_name)| {
            quote! { #mock_fn_name as #fn_name }
        }).collect();
        
        let expanded = quote! {
            #[cfg(not(test))]
            #input
            
            #[cfg(test)]
            use #base_path_tokens::{#(#mock_mappings),*};
        };
        TokenStream::from(expanded)
    }
}
