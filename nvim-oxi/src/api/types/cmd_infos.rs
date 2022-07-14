use derive_builder::Builder;
use nvim_types::{Array, Object};
use serde::Deserialize;

use super::{CmdMagic, CmdRange, CommandAddr, CommandModifiers, CommandNArgs};
use crate::object::{self, de::utils, FromObject, ToObject};

#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Builder, Deserialize)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct CmdInfos {
    /// Value of `:command-addr`. Uses short name.
    #[serde(deserialize_with = "utils::none_literal_is_none")]
    #[builder(setter(strip_option))]
    // Setter doesn't exist bc `addr` is ignored when passed to `nvim_cmd`.
    pub addr: Option<CommandAddr>,

    /// Command arguments.
    #[builder(setter(custom))]
    pub args: Vec<String>,

    /// Whether the command contains a `<bang>` (`!`) modifier.
    #[builder(setter(strip_option))]
    pub bang: Option<bool>,

    /// Command name.
    #[builder(setter(into, strip_option))]
    pub cmd: Option<String>,

    /// Any count that was supplied to the command. `None` if command cannot
    /// take a count.
    #[builder(setter(strip_option))]
    #[serde(deserialize_with = "utils::minus_one_is_none")]
    pub count: Option<u32>,

    #[builder(setter(strip_option))]
    pub magic: Option<CmdMagic>,

    #[builder(setter(strip_option))]
    pub mods: Option<CommandModifiers>,

    // Setter doesn't exist bc `nargs` is ignored when passed to `nvim_cmd`.
    /// Value of `:command-nargs`
    #[builder(setter(skip))]
    pub nargs: Option<CommandNArgs>,

    // Setter doesn't exist bc `nextcmd` is ignored when passed to `nvim_cmd`.
    /// Next command if there are multiple commands separated by a `:bar`.
    /// `None` if there isn't a next command.
    #[builder(setter(skip))]
    #[serde(deserialize_with = "utils::empty_string_is_none")]
    pub nextcmd: Option<String>,

    /// Command range.
    #[builder(setter(strip_option))]
    pub range: Option<CmdRange>,

    /// The optional command `<register>`. `None` if not specified or if
    /// command cannot take a register.
    #[serde(deserialize_with = "utils::char_from_string")]
    #[builder(setter(strip_option))]
    pub reg: Option<char>,
}

impl CmdInfos {
    #[inline(always)]
    pub fn builder() -> CmdInfosBuilder {
        CmdInfosBuilder::default()
    }
}

impl CmdInfosBuilder {
    pub fn args<S, I>(&mut self, iter: I) -> &mut Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        self.args = Some(iter.into_iter().map(Into::into).collect::<Vec<_>>());
        self
    }

    pub fn build(&mut self) -> CmdInfos {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl FromObject for CmdInfos {
    fn from_obj(obj: Object) -> crate::Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}

#[derive(Default, Debug)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_cmd {
    cmd: Object,
    reg: Object,
    bang: Object,
    addr: Object,
    mods: Object,
    args: Object,
    count: Object,
    magic: Object,
    nargs: Object,
    range: Object,
    nextcmd: Object,
}

impl From<&CmdInfos> for KeyDict_cmd {
    fn from(infos: &CmdInfos) -> Self {
        Self {
            cmd: infos.cmd.clone().into(),
            reg: infos.reg.into(),
            bang: infos.bang.into(),
            addr: infos.addr.to_obj().unwrap(),
            mods: infos.mods.to_obj().unwrap(),
            args: Array::from_iter(infos.args.clone()).into(),
            count: infos.count.into(),
            magic: infos.magic.into(),
            nargs: infos.nargs.to_obj().unwrap(),
            range: infos.range.into(),
            nextcmd: infos.nextcmd.clone().into(),
        }
    }
}
