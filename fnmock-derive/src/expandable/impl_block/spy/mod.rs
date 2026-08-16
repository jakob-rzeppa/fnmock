use crate::{expandable::impl_block::ImplExpandable, plan::impl_block::spy::ImplSpyPlan};

impl TryFrom<ImplSpyPlan> for ImplExpandable {
    type Error = syn::Error;

    fn try_from(value: ImplSpyPlan) -> Result<Self, Self::Error> {
        todo!()
    }
}
