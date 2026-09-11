use crate::{
    expandable::function::FunctionExpandable,
    expanded::function::{
        FunctionExpanded,
        from::{accessor::build_accessor, module::build_module},
    },
};

mod accessor;
mod module;

impl TryFrom<FunctionExpandable> for FunctionExpanded {
    type Error = syn::Error;

    fn try_from(value: FunctionExpandable) -> Result<Self, Self::Error> {
        let FunctionExpandable {
            ref vis,

            original,
            ref inline_call,

            ref accessor_name,
            ref accessor_generic_params,

            ref interface_type,

            ref module_name,
            ref module_parts,
        } = value;

        let fn_with_inline_call = original.into_fn_with_inline_call(inline_call);
        let accessor_fn = build_accessor(
            vis,
            accessor_name,
            module_name,
            accessor_generic_params,
            interface_type,
        );
        let module = build_module(vis, module_name, module_parts);

        Ok(FunctionExpanded {
            fn_with_inline_call,
            accessor_fn,
            module,
        })
    }
}
