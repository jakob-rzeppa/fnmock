use crate::{
    expandable::function::FunctionExpandable,
    expanded::function::{
        FunctionExpanded,
        from::{accessor::build_accessor, inline_call::insert_inline_call, module::build_module},
    },
};

mod accessor;
mod inline_call;
mod module;

impl TryFrom<FunctionExpandable> for FunctionExpanded {
    type Error = syn::Error;

    fn try_from(value: FunctionExpandable) -> Result<Self, Self::Error> {
        let FunctionExpandable {
            ref vis,

            mut item_fn,
            ref inline_call,

            ref accessor_name,
            ref accessor_generic_params,

            ref interface_getter,
            ref interface_type,

            ref module_name,
            ref module_parts,
        } = value;

        insert_inline_call(&mut item_fn, inline_call);
        let accessor_fn = build_accessor(
            vis,
            accessor_name,
            accessor_generic_params,
            interface_getter,
            interface_type,
        );
        let module = build_module(vis, module_name, module_parts);

        Ok(FunctionExpanded {
            fn_with_inline_call: item_fn,
            accessor_fn,
            module,
        })
    }
}
