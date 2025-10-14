#[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ProgressMessageStatus {
    /// The progress item completed successfully.
    Success,

    /// The progress is ongoing.
    Running,

    /// The progress item failed.
    Failed,

    /// The progressing process should be canceled.
    Cancel,
}

impl ProgressMessageStatus {
    #[inline]
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Running => "running",
            Self::Failed => "failed",
            Self::Cancel => "cancel",
        }
    }
}
