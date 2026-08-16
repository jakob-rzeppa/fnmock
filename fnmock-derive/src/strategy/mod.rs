pub mod function {
    pub mod fake;
    pub mod spy;
}
pub mod impl_block {
    pub mod fake;
    pub mod spy;
}

pub trait Strategy {
    /// The original item to be processed.
    type Item;

    /// The information extracted from the original item.
    type ItemInfo: TryFrom<Self::Item, Error = syn::Error>;

    /// The plan derived from the item information.
    type Plan: TryFrom<Self::ItemInfo, Error = syn::Error>;

    /// The expandable representation of the plan.
    type Expandable: TryFrom<Self::Plan, Error = syn::Error>;

    /// The final expanded output after processing the expandable representation.
    type Expanded: TryFrom<Self::Expandable, Error = syn::Error> + Into<proc_macro2::TokenStream>;
}

pub fn execute<S: Strategy>(item: S::Item) -> Result<proc_macro2::TokenStream, syn::Error> {
    let item_info = S::ItemInfo::try_from(item)?;
    let plan = S::Plan::try_from(item_info)?;
    let expandable = S::Expandable::try_from(plan)?;
    let expanded = S::Expanded::try_from(expandable)?;
    Ok(expanded.into())
}
