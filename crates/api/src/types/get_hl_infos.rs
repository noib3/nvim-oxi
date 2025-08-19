use types::String as NvimString;

use super::HighlightInfos;
use crate::SuperIterator;

/// The type returned by [`get_hl`](crate::get_hl).
pub enum GetHlInfos<Map: SuperIterator<(NvimString, HighlightInfos)>> {
    /// The [`HighlightInfos`] relative to a specific highlight group.
    ///
    /// This is returned when either [`id`] or [`name`] are specified.
    ///
    /// [`id`]: crate::opts::GetHighlightOptsBuilder::id
    /// [`name`]: crate::opts::GetHighlightOptsBuilder::name
    Single(HighlightInfos),

    /// A map from highlight group name to [`HighlightInfos`] for that group.
    ///
    /// This is returned when neither [`id`] nor [`name`] are specified.
    ///
    /// [`id`]: crate::opts::GetHighlightOptsBuilder::id
    /// [`name`]: crate::opts::GetHighlightOptsBuilder::name
    Map(Map),
}
