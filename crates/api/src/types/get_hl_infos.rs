use types::String as NvimString;

use super::HighlightInfos;
use crate::SuperIterator;

/// The type returned by [`get_hl`](crate::get_hl).
pub enum GetHlInfos<Map: SuperIterator<(NvimString, HighlightInfos)>> {
    /// The [`HighlightInfos`] relative to a specific highlight group.
    /// This is returned when either [`id`](crate::opts::GetHlOptsBuilder::id)
    /// or [`name`](crate::opts::GetHlOptsBuilder::name) are specified.
    Single(HighlightInfos),

    /// A map from highlight group name to [`HighlightInfos`] for that group.
    /// This is returned when neither [`id`](crate::opts::GetHlOptsBuilder::id)
    /// nor [`name`](crate::opts::GetHlOptsBuilder::name) are specified.
    Map(Map),
}
