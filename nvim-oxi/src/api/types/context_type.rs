use nvim_types::String as NvimString;
use serde::Serialize;

use crate::object;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum ContextType {
    #[serde(rename = "bufs")]
    Bufferlist,

    #[serde(rename = "gvars")]
    GlobalVars,

    #[serde(rename = "funcs")]
    GlobalAndScriptLocalFuncs,

    #[serde(rename = "jumps")]
    Jumplist,

    #[serde(rename = "regs")]
    Registers,

    #[serde(rename = "sfuncs")]
    ScriptLocalFuncs,
}

impl From<ContextType> for NvimString {
    fn from(ctx: ContextType) -> Self {
        ctx.serialize(object::Serializer)
            .expect("`ContextType` is serializable")
            .try_into()
            .expect("`ContextType` is serialized into a string")
    }
}
