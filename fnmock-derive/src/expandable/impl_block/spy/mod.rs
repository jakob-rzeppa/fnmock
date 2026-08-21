use crate::{expandable::impl_block::ImplExpandable, scheme::impl_block::spy::ImplSpyScheme};

impl TryFrom<ImplSpyScheme> for ImplExpandable {
    type Error = syn::Error;

    fn try_from(_value: ImplSpyScheme) -> Result<Self, Self::Error> {
        todo!()
    }
}
