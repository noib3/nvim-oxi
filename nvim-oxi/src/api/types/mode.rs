use nvim_types::string::String as NvimString;
use serde::{Deserialize, Serialize};

/// TODO: docs
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Mode {
    CmdLine,
    Insert,
    Langmap,
    NormalVisualOperator,
    Normal,
    OperatorPending,
    Select,
    Terminal,
    Visual,
    VisualSelect,
}

impl From<Mode> for NvimString {
    fn from(mode: Mode) -> Self {
        use Mode::*;
        match mode {
            CmdLine => "c",
            Insert => "i",
            Langmap => "l",
            Normal => "n",
            NormalVisualOperator => "",
            OperatorPending => "o",
            Select => "s",
            Terminal => "t",
            Visual => "x",
            VisualSelect => "v",
        }
        .into()
    }
}
