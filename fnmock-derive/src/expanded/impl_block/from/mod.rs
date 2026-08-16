use std::collections::HashMap;

use crate::{
    expandable::impl_block::ImplExpandable,
    expanded::impl_block::{
        ImplExpanded,
        from::{
            accessors::{AccessorMethodInfo, build_accessor_impl},
            inline_calls::insert_inline_calls,
            modules::build_modules,
        },
    },
};

mod accessors;
mod inline_calls;
mod modules;

impl TryFrom<ImplExpandable> for ImplExpanded {
    type Error = syn::Error;

    fn try_from(value: ImplExpandable) -> Result<Self, Self::Error> {
        let ImplExpandable {
            mut item_impl,
            ref struct_name,
            ref struct_generic_params,
            ref struct_generic_idents,
            ref methods,
        } = value;

        let mut inline_calls_map: HashMap<syn::Ident, syn::Block> = HashMap::new();
        for (method_name, method) in methods {
            inline_calls_map.insert(method_name.clone(), method.inline_call.clone());
        }
        insert_inline_calls(&mut item_impl, &inline_calls_map);

        let accessor_method_infos = methods
            .values()
            .map(|method| AccessorMethodInfo {
                vis: &method.vis,
                name: &method.accessor_name,
                method_generic_params: &method.method_generic_params,
                interface_getter: &method.interface_getter,
                interface_type: &method.interface_type,
            })
            .collect::<Vec<AccessorMethodInfo>>();
        let accessor = build_accessor_impl(
            struct_name,
            struct_generic_params,
            struct_generic_idents,
            &accessor_method_infos,
        );

        let modules_info = methods
            .iter()
            .map(|(_, method)| modules::ModuleInfo {
                vis: &method.vis,
                name: &method.module_name,
                parts: &method.module_parts,
            })
            .collect::<Vec<modules::ModuleInfo>>();
        let modules = build_modules(&modules_info);

        Ok(ImplExpanded {
            impl_with_inline_calls: item_impl,
            accessor_impl_block: accessor,
            modules,
        })
    }
}

#[cfg(test)]
mod tests {
    use quote::{ToTokens, quote};
    use syn::parse_quote;

    use super::*;
    use crate::expandable::impl_block::ImplMethodExpandable;

    #[test]
    fn test_try_from_impl_expandable_multiple_methods_with_generics() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl<S: Display + 'static> MyStruct<S> {
                fn method_one(&self) -> i32 {
                    1
                }

                pub fn method_two<T: Display + 'static>(&self, t: T) -> String {
                    format!("{} {}", self.0, t)
                }
            }
        };

        let mut methods = HashMap::new();
        methods.insert(
            parse_quote!(method_one),
            ImplMethodExpandable {
                vis: syn::Visibility::Inherited,
                inline_call: parse_quote!({
                    inline_call();
                }),
                accessor_name: parse_quote!(method_one_fake),
                method_generic_params: vec![],
                interface_getter: parse_quote!(self::method_one_module::interface::<S>()),
                interface_type: parse_quote!(InterfaceOne<S>),
                module_name: parse_quote!(method_one_module),
                module_parts: vec![quote! {
                    pub fn interface() -> InterfaceOne<S> {
                        InterfaceOne {}
                    }
                }],
            },
        );
        methods.insert(
            parse_quote!(method_two),
            ImplMethodExpandable {
                vis: parse_quote!(pub),
                inline_call: parse_quote!({
                    inline_call();
                }),
                accessor_name: parse_quote!(method_two_fake),
                method_generic_params: vec![parse_quote!(T: Display + 'static)],
                interface_getter: parse_quote!(self::method_two_module::interface::<S, T>()),
                interface_type: parse_quote!(InterfaceTwo<S, T>),
                module_name: parse_quote!(method_two_module),
                module_parts: vec![quote! {
                    pub fn interface() -> InterfaceTwo<S, T> {
                        InterfaceTwo {}
                    }
                }],
            },
        );

        let expandable = ImplExpandable {
            item_impl,
            struct_name: parse_quote!(MyStruct),
            struct_generic_params: vec![parse_quote!(S: Display + 'static)],
            struct_generic_idents: vec![parse_quote!(S)],
            methods,
        };

        let expanded = ImplExpanded::try_from(expandable).unwrap();

        // The methods are copied in-place, so this order is deterministic.
        let expected_impl_with_inline_calls: syn::ItemImpl = parse_quote! {
            impl<S: Display + 'static> MyStruct<S> {
                fn method_one(&self) -> i32 {
                    #[cfg(test)]
                    {
                        inline_call();
                    }

                    {
                        1
                    }
                }

                pub fn method_two<T: Display + 'static>(&self, t: T) -> String {
                    #[cfg(test)]
                    {
                        inline_call();
                    }

                    {
                        format!("{} {}", self.0, t)
                    }
                }
            }
        };
        assert_eq!(
            expanded
                .impl_with_inline_calls
                .to_token_stream()
                .to_string(),
            expected_impl_with_inline_calls
                .to_token_stream()
                .to_string()
        );

        let expected_accessor_impl_block: syn::ItemImpl = parse_quote! {
            impl<S: Display + 'static> MyStruct<S> {
                fn method_one_fake() -> InterfaceOne<S> {
                    self::method_one_module::interface::<S>()
                }

                pub fn method_two_fake<T: Display + 'static>() -> InterfaceTwo<S, T> {
                    self::method_two_module::interface::<S, T>()
                }
            }
        };
        // `methods` is a HashMap, so the order of the generated accessor methods
        // is not guaranteed; compare them as an order-independent set.
        assert_eq!(
            normalize_impl(&expanded.accessor_impl_block),
            normalize_impl(&expected_accessor_impl_block)
        );

        let expected_module_one: syn::ItemMod = parse_quote! {
            #[cfg(test)]
            mod method_one_module {
                pub fn interface() -> InterfaceOne<S> {
                    InterfaceOne {}
                }
            }
        };
        let expected_module_two: syn::ItemMod = parse_quote! {
            #[cfg(test)]
            pub mod method_two_module {
                pub fn interface() -> InterfaceTwo<S, T> {
                    InterfaceTwo {}
                }
            }
        };
        // `methods` is a HashMap, so the order of the generated modules is not
        // guaranteed; compare them as an order-independent set.
        let mut actual_modules: Vec<String> = expanded
            .modules
            .iter()
            .map(|m| m.to_token_stream().to_string())
            .collect();
        actual_modules.sort();
        let mut expected_modules = vec![
            expected_module_one.to_token_stream().to_string(),
            expected_module_two.to_token_stream().to_string(),
        ];
        expected_modules.sort();
        assert_eq!(actual_modules, expected_modules);
    }

    /// Splits an `ItemImpl` into its header (everything but the items) and its
    /// items, sorted, so impls can be compared regardless of item order.
    fn normalize_impl(item_impl: &syn::ItemImpl) -> (String, Vec<String>) {
        let mut header = item_impl.clone();
        header.items.clear();

        let mut items: Vec<String> = item_impl
            .items
            .iter()
            .map(|item| item.to_token_stream().to_string())
            .collect();
        items.sort();

        (header.to_token_stream().to_string(), items)
    }
}
