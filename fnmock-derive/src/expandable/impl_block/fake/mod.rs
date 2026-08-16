use crate::{expandable::impl_block::ImplExpandable, plan::impl_block::fake::ImplFakePlan};

impl TryFrom<ImplFakePlan> for ImplExpandable {
    type Error = syn::Error;

    fn try_from(value: ImplFakePlan) -> Result<Self, Self::Error> {
        todo!()
    }
}
