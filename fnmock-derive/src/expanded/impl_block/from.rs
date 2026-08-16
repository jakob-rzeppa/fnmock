use crate::{expandable::impl_block::ImplExpandable, expanded::impl_block::ImplExpanded};

impl TryFrom<ImplExpandable> for ImplExpanded {
    type Error = syn::Error;

    fn try_from(value: ImplExpandable) -> Result<Self, Self::Error> {
        todo!()
    }
}
