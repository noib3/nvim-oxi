use oxi_types as nvim;
use serde::Deserialize;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum Mode {
    #[serde(rename = "c")]
    CmdLine,

    #[serde(rename = "i")]
    Insert,

    #[serde(rename = "!")]
    InsertCmdLine,

    #[serde(rename = "l")]
    Langmap,

    #[serde(rename(deserialize = " "))]
    NormalVisualOperator,

    #[serde(rename = "n")]
    Normal,

    #[serde(rename = "o")]
    OperatorPending,

    #[serde(rename = "s")]
    Select,

    #[serde(rename = "t")]
    Terminal,

    #[serde(rename = "x")]
    Visual,

    #[serde(rename = "v")]
    VisualSelect,
}

macro_rules! is_mode {
    ($fn_name:ident, $variant:ident) => {
        #[inline(always)]
        pub const fn $fn_name(&self) -> bool {
            matches!(self, Mode::$variant)
        }
    };
}

impl Mode {
    is_mode!(is_cmd_line, CmdLine);
    is_mode!(is_insert, Insert);
    is_mode!(is_langmap, Langmap);
    is_mode!(is_nvo, NormalVisualOperator);
    is_mode!(is_normal, Normal);
    is_mode!(is_op_pending, OperatorPending);
    is_mode!(is_select, Select);
    is_mode!(is_terminal, Terminal);
    is_mode!(is_visual, Visual);
    is_mode!(is_visual_select, VisualSelect);
}

impl From<Mode> for nvim::String {
    fn from(mode: Mode) -> Self {
        use Mode::*;
        match mode {
            CmdLine => "c",
            Insert => "i",
            InsertCmdLine => "!",
            Langmap => "l",
            NormalVisualOperator => "",
            Normal => "n",
            OperatorPending => "o",
            Select => "s",
            Terminal => "t",
            Visual => "x",
            VisualSelect => "v",
        }
        .into()
    }
}
