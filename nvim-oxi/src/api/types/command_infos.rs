use nvim_types::LuaRef;

use super::{CommandAddr, CommandNArgs, CommandRange};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CommandInfos {
    pub addr: Option<CommandAddr>,
    pub bang: bool,
    pub bar: bool,
    pub callback: Option<LuaRef>,
    pub complete: Option<String>,
    pub complete_arg: Option<String>,
    pub count: Option<String>,
    pub definition: Option<String>,
    pub keepscript: bool,
    pub name: String,
    pub nargs: Option<CommandNArgs>,
    pub range: Option<CommandRange>,
    pub register: bool,
    pub script_id: u32,
}
