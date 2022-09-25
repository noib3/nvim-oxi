use nvim_types::{self as nvim, FromObject, Serializer};
use serde::Serialize;

#[non_exhaustive]
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

impl From<ContextType> for nvim::String {
    fn from(ctx: ContextType) -> Self {
        nvim::String::from_obj(
            ctx.serialize(Serializer::new())
                .expect("`ContextType` is serializable"),
        )
        .expect("`ContextType` is serialized into a string")
    }
}
