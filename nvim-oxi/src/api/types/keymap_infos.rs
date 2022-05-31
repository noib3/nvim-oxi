use nvim_types::LuaRef;
use serde::Deserialize;

use super::Mode;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct KeymapInfos {
    buffer: bool,
    callback: Option<LuaRef>,
    expr: bool,
    lhs: String,
    lnum: Option<u32>, // none if 0
    mode: Mode,
    noremap: bool,
    nowait: bool,
    rhs: Option<String>, // none if empty
    script: bool,
    sid: i32,
    silent: bool,
}
