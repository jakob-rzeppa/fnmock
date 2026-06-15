use crate::fakeable::{ handle_fakeable };

mod fakeable;

#[proc_macro_attribute]
pub fn fakeable(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream
) -> proc_macro::TokenStream {
    let res = handle_fakeable(attr.into(), item.into());

    match res {
        Ok(expanded) => expanded.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

use quote::{ format_ident, quote };
use syn::{
    parse::{ Parse, ParseStream },
    parse_macro_input,
    token::Comma,
    Ident,
    Result,
    Token,
    TypePath,
};

struct FakeInput {
    ty: TypePath,
    ty_generics: Option<syn::AngleBracketedGenericArguments>,
    method: Ident,
    method_generics: Option<syn::AngleBracketedGenericArguments>,
}

impl Parse for FakeInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut tokens = proc_macro2::TokenStream::new();

        while !input.is_empty() && !input.peek(Token![<]) && !input.peek(Token![,]) {
            let tt: proc_macro2::TokenTree = input.parse()?;
            tokens.extend(std::iter::once(tt));
        }

        let ty: syn::TypePath = syn::parse2(tokens)?;

        let ty_generics = if input.peek(Token![<]) { Some(input.parse()?) } else { None };

        input.parse::<Comma>()?;

        let method: Ident = input.parse()?;

        let method_generics = if input.peek(Token![<]) { Some(input.parse()?) } else { None };

        Ok(Self {
            ty,
            ty_generics,
            method,
            method_generics,
        })
    }
}

#[proc_macro]
pub fn fake(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let FakeInput { ty, ty_generics, method, method_generics } = parse_macro_input!(
        input as FakeInput
    );

    let fake_method = format_ident!("{}_fake", method);

    let expanded = match (ty_generics, method_generics) {
        (Some(ty_g), Some(method_g)) => {
            quote! {
                #ty::#ty_g::#fake_method::#method_g()
            }
        }
        (Some(ty_g), None) => {
            quote! {
                #ty::#ty_g::#fake_method()
            }
        }
        (None, Some(method_g)) => {
            quote! {
                #ty::#fake_method::#method_g()
            }
        }
        (None, None) => {
            quote! {
                #ty::#fake_method()
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
