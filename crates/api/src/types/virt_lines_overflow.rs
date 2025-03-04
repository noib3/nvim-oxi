#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VirtLinesOverflow {
    Scroll,
    Trunc,
}

impl VirtLinesOverflow {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Scroll => "scroll",
            Self::Trunc => "trunk",
        }
    }
}

impl From<VirtLinesOverflow> for types::String {
    fn from(virt_lines_overflow: VirtLinesOverflow) -> Self {
        virt_lines_overflow.as_str().into()
    }
}
