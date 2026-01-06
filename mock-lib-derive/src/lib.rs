use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Type};
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

#[proc_macro_attribute]
pub fn function_mock(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    // Extract function details
    let fn_visibility = input.vis.clone();
    let fn_name = input.sig.ident.clone();
    let fn_inputs = input.sig.inputs.clone();
    let fn_output = input.sig.output.clone();
    let fn_block = input.block.clone();

    // Extract parameter types and names
    let param_types: Vec<_> = input.sig.inputs.iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => Some(&pat_type.ty),
            _ => panic!("self parameters not supported"),
        })
        .collect();
    
    let param_names: Vec<_> = input.sig.inputs.iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => Some(&pat_type.pat),
            _ => panic!("self parameters not supported"),
        })
        .collect();

    let params_type = create_param_type(&input.sig.inputs);
    
    // Extract return type from ReturnType
    let return_type: Type = match &input.sig.output {
        syn::ReturnType::Default => syn::parse2(quote! { () }).unwrap(),
        syn::ReturnType::Type(_, ty) => (**ty).clone(),
    };

    // Create function pointer type
    let function_pointer_declaration = quote! {
        fn(#(#param_types),*) #fn_output
    };
    
    // Generate the call arguments - single param vs tuple
    let call_args = if param_names.len() == 1 {
        quote! { #(#param_names)* }
    } else {
        quote! { (#(#param_names),*) }
    };

    // The name of the instance of the FunctionMock struct
    let mock_name = format!("{}_MOCK", fn_name.to_string().to_uppercase());
    let mock_name = syn::Ident::new(&mock_name, fn_name.span());

    let params_type_name = format!("{}Params", to_pascal_case(&fn_name.to_string()));
    let params_type_name = syn::Ident::new(&params_type_name, fn_name.span());
    
    let return_type_name = format!("{}Result", to_pascal_case(&fn_name.to_string()));
    let return_type_name = syn::Ident::new(&return_type_name, fn_name.span());
    
    let function_type_name = format!("{}Function", to_pascal_case(&fn_name.to_string()));
    let function_type_name = syn::Ident::new(&function_type_name, fn_name.span());

    // Generate both the original function and the mock module
    let expanded = quote! {
        #fn_visibility fn #fn_name(#fn_inputs) #fn_output #fn_block

        #[cfg(test)]
        pub(crate) mod mock {
            use std::cell::RefCell;
            use mock_lib::function_mock::FunctionMock;

            type #function_type_name = #function_pointer_declaration;
            type #params_type_name = #params_type;
            type #return_type_name = #return_type;

            thread_local! {
                static #mock_name: RefCell<FunctionMock<
                    #function_type_name,
                    #params_type_name,
                    #return_type_name
                >> = RefCell::new(FunctionMock::new(stringify!(#fn_name)));
            }

            pub fn #fn_name(#fn_inputs) #fn_output {
                #mock_name.with(|mock| {
                    let mut mock = mock.borrow_mut();
                    mock.call(#call_args)
                })
            }

            pub(crate) mod #fn_name {
                pub(crate) fn mock_implementation(new_f: super::#function_type_name) {
                    super::#mock_name.with(|mock| { mock.borrow_mut().mock_implementation(new_f) })
                }

                pub(crate) fn clear_mock() {
                    super::#mock_name.with(|mock|{ mock.borrow_mut().clear_mock() })
                }

                pub(crate) fn assert_times(expected_num_of_calls: u32) {
                    super::#mock_name.with(|mock| { mock.borrow().assert_times(expected_num_of_calls) })
                }

                pub(crate) fn assert_with(params: super::#params_type_name) {
                    super::#mock_name.with(|mock| { mock.borrow().assert_with(&params) })
                }
            }
        }
    };

    TokenStream::from(expanded)
}
