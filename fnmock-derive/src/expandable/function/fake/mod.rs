use crate::{expandable::function::FunctionExpandable, plan::function::fake::FunctionFakePlan};

impl TryFrom<FunctionFakePlan> for FunctionExpandable {
    type Error = syn::Error;

    fn try_from(value: FunctionFakePlan) -> Result<Self, Self::Error> {
        todo!()
    }
}
