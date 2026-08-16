use crate::{expandable::impl_block::ImplExpandable, scheme::impl_block::fake::ImplFakeScheme};

impl TryFrom<ImplFakeScheme> for ImplExpandable {
    type Error = syn::Error;

    fn try_from(value: ImplFakeScheme) -> Result<Self, Self::Error> {
        todo!()
    }
}
