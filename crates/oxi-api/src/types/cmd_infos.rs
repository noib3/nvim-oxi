use oxi_types::{
    conversion::{self, FromObject, ToObject},
    serde::Deserializer,
    Array,
    Object,
};
use serde::Deserialize;

use super::{CmdMagic, CmdRange, CommandAddr, CommandModifiers, CommandNArgs};
use crate::serde_utils as utils;

#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize)]
pub struct CmdInfos {
    /// Value of `:command-addr`. Uses short name.
    #[serde(default, deserialize_with = "utils::none_literal_is_none")]
    // No setter bc `addr` is ignored when passed to `nvim_cmd`.
    pub addr: Option<CommandAddr>,

    /// Command arguments.
    #[serde(default)]
    pub args: Vec<String>,

    /// Whether the command contains a `<bang>` (`!`) modifier.
    #[serde(default)]
    pub bang: Option<bool>,

    /// Command name.
    #[serde(default)]
    pub cmd: Option<String>,

    /// Any count that was supplied to the command. `None` if command cannot
    /// take a count.
    #[serde(default, deserialize_with = "utils::minus_one_is_none")]
    pub count: Option<u32>,

    #[serde(default)]
    pub magic: Option<CmdMagic>,

    #[serde(default)]
    pub mods: Option<CommandModifiers>,

    // Setter doesn't exist bc `nargs` is ignored when passed to `nvim_cmd`.
    /// Value of `:command-nargs`
    #[serde(default)]
    pub nargs: Option<CommandNArgs>,

    // Setter doesn't exist bc `nextcmd` is ignored when passed to `nvim_cmd`.
    /// Next command if there are multiple commands separated by a `:bar`.
    /// `None` if there isn't a next command.
    #[serde(default, deserialize_with = "utils::empty_string_is_none")]
    pub nextcmd: Option<String>,

    /// Command range.
    #[serde(default)]
    pub range: Option<CmdRange>,

    /// The optional command `<register>`. `None` if not specified or if
    /// command cannot take a register.
    #[serde(default, deserialize_with = "utils::char_from_string")]
    pub reg: Option<char>,
}

impl CmdInfos {
    #[inline(always)]
    pub fn builder() -> CmdInfosBuilder {
        CmdInfosBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct CmdInfosBuilder(CmdInfos);

impl CmdInfosBuilder {
    /// Command arguments.
    #[inline]
    pub fn args<S, I>(&mut self, iter: I) -> &mut Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        self.0.args = iter.into_iter().map(Into::into).collect();
        self
    }

    /// Whether the command contains a `<bang>` (`!`) modifier.
    #[inline]
    pub fn bang(&mut self, bang: bool) -> &mut Self {
        self.0.bang = Some(bang);
        self
    }

    /// Command name.
    #[inline]
    pub fn cmd(&mut self, cmd: impl Into<String>) -> &mut Self {
        self.0.cmd = Some(cmd.into());
        self
    }

    /// Any count that was supplied to the command. `None` if command cannot
    /// take a count.
    #[inline]
    pub fn count(&mut self, count: u32) -> &mut Self {
        self.0.count = Some(count);
        self
    }

    #[inline]
    pub fn magic(&mut self, magic: CmdMagic) -> &mut Self {
        self.0.magic = Some(magic);
        self
    }

    #[inline]
    pub fn mods(&mut self, mods: CommandModifiers) -> &mut Self {
        self.0.mods = Some(mods);
        self
    }

    // Setter doesn't exist bc `nargs` is ignored when passed to `nvim_cmd`.
    /// Value of `:command-nargs`
    #[inline]
    pub fn nargs(&mut self, nargs: CommandNArgs) -> &mut Self {
        self.0.nargs = Some(nargs);
        self
    }

    // Setter doesn't exist bc `nextcmd` is ignored when passed to `nvim_cmd`.
    /// Next command if there are multiple commands separated by a `:bar`.
    /// `None` if there isn't a next command.
    #[inline]
    pub fn nextcmd(&mut self, nextcmd: impl Into<String>) -> &mut Self {
        self.0.nextcmd = Some(nextcmd.into());
        self
    }

    /// Command range.
    #[inline]
    pub fn range(&mut self, range: CmdRange) -> &mut Self {
        self.0.range = Some(range);
        self
    }

    /// The optional command `<register>`. `None` if not specified or if
    /// command cannot take a register.
    #[inline]
    pub fn reg(&mut self, reg: char) -> &mut Self {
        self.0.reg = Some(reg);
        self
    }

    #[inline]
    pub fn build(&mut self) -> CmdInfos {
        std::mem::take(&mut self.0)
    }
}

impl FromObject for CmdInfos {
    #[inline]
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

#[cfg(not(feature = "neovim-nightly"))]
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

#[cfg(feature = "neovim-nightly")]
#[derive(Default, Debug)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_cmd {
    cmd: Object,
    range: Object,
    count: Object,
    reg: Object,
    bang: Object,
    args: Object,
    magic: Object,
    mods: Object,
    nargs: Object,
    addr: Object,
    nextcmd: Object,
}

impl From<&CmdInfos> for KeyDict_cmd {
    #[inline]
    fn from(infos: &CmdInfos) -> Self {
        Self {
            cmd: infos.cmd.clone().into(),
            reg: infos.reg.into(),
            bang: infos.bang.into(),
            addr: infos
                .addr
                .map(|v| v.to_object().unwrap())
                .unwrap_or_default(),
            mods: infos
                .mods
                .map(|v| v.to_object().unwrap())
                .unwrap_or_default(),
            args: Array::from_iter(infos.args.clone()).into(),
            count: infos.count.into(),
            magic: infos
                .magic
                .map(|v| v.to_object().unwrap())
                .unwrap_or_default(),
            nargs: infos
                .nargs
                .map(|v| v.to_object().unwrap())
                .unwrap_or_default(),
            range: infos.range.into(),
            nextcmd: infos.nextcmd.clone().into(),
        }
    }
}
