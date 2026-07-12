use quote::{ ToTokens, quote };

use crate::{
    extract::{ function::info::FunctionInfo, item_impl::info::ImplItemFnInfo },
    names::{
        NameType,
        build_impl_interface_struct_name,
        build_impl_module_name,
        build_interface_struct_name,
        build_module_name,
    },
};

#[derive(Clone)]
pub struct InlineCallInfo {
    pub module_name: syn::Ident,
    pub interface_struct_name: syn::Ident,
    pub fake_call_values: Vec<FakeCallValue>,
    pub generic_types: Option<Vec<syn::Type>>,
}

#[derive(Clone)]
pub enum FakeCallValue {
    Ident(syn::Ident),
    Tuple(Vec<FakeCallValue>),
}

impl TryFrom<&FunctionInfo> for InlineCallInfo {
    type Error = syn::Error;

    fn try_from(function_info: &FunctionInfo) -> Result<Self, Self::Error> {
        let module_name = build_module_name(&function_info.name, NameType::Fake);
        let interface_struct_name = build_interface_struct_name(
            &function_info.name,
            NameType::Fake
        );

        let fake_call_values = function_info.param_pats
            .iter()
            .map(FakeCallValue::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let generic_types = if let Some(generic_info) = &function_info.generic_info {
            Some(generic_info.types.clone())
        } else {
            None
        };

        Ok(InlineCallInfo {
            module_name,
            interface_struct_name,
            fake_call_values,
            generic_types,
        })
    }
}

impl TryFrom<&ImplItemFnInfo> for InlineCallInfo {
    type Error = syn::Error;

    fn try_from(impl_item_fn_info: &ImplItemFnInfo) -> Result<Self, Self::Error> {
        let module_name = build_impl_module_name(
            &impl_item_fn_info.struct_name,
            &impl_item_fn_info.method_name,
            NameType::Fake
        );
        let interface_struct_name = build_impl_interface_struct_name(
            &impl_item_fn_info.struct_name,
            &impl_item_fn_info.method_name,
            NameType::Fake
        );

        let fake_call_values = impl_item_fn_info.param_pats
            .iter()
            .map(FakeCallValue::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let generic_types = if let Some(generic_info) = &impl_item_fn_info.generic_info {
            Some(generic_info.types.clone())
        } else {
            None
        };

        Ok(InlineCallInfo {
            module_name,
            interface_struct_name,
            fake_call_values,
            generic_types,
        })
    }
}

impl ToTokens for FakeCallValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FakeCallValue::Ident(ident) => {
                ident.to_tokens(tokens);
            }
            FakeCallValue::Tuple(elements) => {
                let element_tokens = elements.iter().map(|e| e.to_token_stream());
                let tuple_tokens = quote! { (#(#element_tokens),*) };
                tuple_tokens.to_tokens(tokens);
            }
        }
    }
}

impl TryFrom<&syn::Pat> for FakeCallValue {
    type Error = syn::Error;

    fn try_from(pat: &syn::Pat) -> Result<Self, Self::Error> {
        match pat {
            syn::Pat::Ident(pat_ident) => {
                // If the pattern uses `ref ident`, we cannot use it in the fakes, since the signature of the fake function will need a value, not a reference and we cannot obtain a value from a reference in the general case.
                if pat_ident.by_ref.is_some() {
                    return Err(
                        syn::Error::new_spanned(
                            pat_ident,
                            "The `ref` keyword is not supported for fake call values. Please use the identifier directly without `ref` (e.g. `ident` instead of `ref ident`)."
                        )
                    );
                }

                // We need to ignore the mutability in the pattern and just get the identifier name for the fake call value.
                Ok(FakeCallValue::Ident(pat_ident.ident.clone()))
            }
            syn::Pat::Tuple(pat_tuple) => {
                let elements = pat_tuple.elems
                    .iter()
                    .map(FakeCallValue::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(FakeCallValue::Tuple(elements))
            }
            syn::Pat::Struct(pat_struct) =>
                Err(
                    syn::Error::new_spanned(
                        pat_struct,
                        "Struct destructuring patterns are not supported for fake call values"
                    )
                ),
            syn::Pat::TupleStruct(pat_tuple_struct) =>
                Err(
                    syn::Error::new_spanned(
                        pat_tuple_struct,
                        "Tuple struct destructuring patterns are not supported for fake call values"
                    )
                ),
            syn::Pat::Macro(pat_macro) =>
                Err(
                    syn::Error::new_spanned(
                        pat_macro,
                        "Macro patterns are not supported for fake call values"
                    )
                ),
            _ => Err(syn::Error::new_spanned(pat, "Unsupported pattern type for fake call values")),
        }
    }
}
