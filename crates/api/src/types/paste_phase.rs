#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PastePhase {
    /// Paste in a single call (i.e. without streaming).
    SingleCall = -1,

    /// Starts the paste. If calling [`paste`](crate::paste) sequentially only
    /// the first call should have this value as `phase`.
    StartPaste = 1,

    /// Continues the paste.
    ContinuePasting = 2,

    /// Ends the paste. If calling [`paste`](crate::paste) sequentially only
    /// the last call should have this value as `phase`.
    EndPaste = 3,
}
