use crate::{expandable::function::FunctionExpandable, plan::function::spy::FunctionSpyPlan};

impl TryFrom<FunctionSpyPlan> for FunctionExpandable {
    type Error = syn::Error;

    fn try_from(value: FunctionSpyPlan) -> Result<Self, Self::Error> {
        todo!()
    }
}
