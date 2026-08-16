use crate::{expandable::function::FunctionExpandable, expanded::function::FunctionExpanded};

impl TryFrom<FunctionExpandable> for FunctionExpanded {
    type Error = syn::Error;

    fn try_from(value: FunctionExpandable) -> Result<Self, Self::Error> {
        todo!()
    }
}
