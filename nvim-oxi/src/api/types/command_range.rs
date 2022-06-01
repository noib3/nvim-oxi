use serde::{Deserialize, Serialize};

/// See `:h command-range` for details.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum CommandRange {
    CurrentLine, // true
    WholeFile,   // "%"
    Count(u32),  // {0}
}
