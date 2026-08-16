use crate::{expandable::function::FunctionExpandable, scheme::function::fake::FunctionFakeScheme};

impl TryFrom<FunctionFakeScheme> for FunctionExpandable {
    type Error = syn::Error;

    fn try_from(value: FunctionFakeScheme) -> Result<Self, Self::Error> {
        todo!()
    }
}
