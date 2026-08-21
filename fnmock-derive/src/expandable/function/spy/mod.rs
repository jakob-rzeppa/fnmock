use crate::{expandable::function::FunctionExpandable, scheme::function::spy::FunctionSpyScheme};

impl TryFrom<FunctionSpyScheme> for FunctionExpandable {
    type Error = syn::Error;

    fn try_from(_value: FunctionSpyScheme) -> Result<Self, Self::Error> {
        todo!()
    }
}
