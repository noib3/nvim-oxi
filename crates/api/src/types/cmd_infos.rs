use serde::Deserialize;
use types::{
    conversion::{self, FromObject, ToObject},
    serde::Deserializer,
    Array,
    Object,
};
#[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
use types::{Boolean, Dictionary, Integer, String as NvimString};

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

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
#[derive(Default, Debug)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct ParseCmdOutput {
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

#[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
#[derive(Default, Debug, Clone, macros::OptsBuilder)]
#[repr(C)]
pub(crate) struct ParseCmdOutput {
    #[builder(mask)]
    mask: u64,
    cmd: NvimString,
    range: Array,
    count: Integer,
    reg: NvimString,
    bang: Boolean,
    args: Array,
    magic: Dictionary,
    mods: Dictionary,
    nargs: Object,

    // Only on 0.10.
    #[cfg(all(feature = "neovim-0-10", not(feature = "neovim-nightly")))]
    addr: Object,

    // Only on Nightly.
    #[cfg(feature = "neovim-nightly")]
    addr: NvimString,

    // Only on Nightly.
    #[cfg(all(feature = "neovim-0-10", not(feature = "neovim-nightly")))]
    nextcmd: Object,

    // Only on 0.10.
    #[cfg(feature = "neovim-nightly")]
    nextcmd: NvimString,
}

impl From<&CmdInfos> for ParseCmdOutput {
    #[inline]
    fn from(infos: &CmdInfos) -> Self {
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        {
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
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        {
            let mut builder = Self::builder();

            if let Some(cmd) = infos.cmd.as_deref() {
                builder.cmd(cmd.into());
            }

            if let Some(range) = infos.range {
                builder.range(Array::from(range));
            }

            if let Some(count) = infos.count {
                builder.count(count as Integer);
            }

            if let Some(reg) = infos.reg {
                builder.reg(reg.into());
            }

            if let Some(bang) = infos.bang {
                builder.bang(bang);
            }

            if !infos.args.is_empty() {
                builder.args(Array::from_iter(infos.args.clone()));
            }

            if let Some(magic) = infos.magic {
                builder.magic(Dictionary::from(magic));
            }

            if let Some(mods) = infos.mods {
                builder.mods(Dictionary::from(mods));
            }

            if let Some(nargs) = infos.nargs {
                builder.nargs(nargs.to_object().unwrap());
            }

            if let Some(addr) = infos.addr {
                builder.addr(addr.as_str().into());
            }

            if let Some(nextcmd) = infos.nextcmd.as_deref() {
                builder.nextcmd(nextcmd.into());
            };

            builder.build()
        }
    }
}

#[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
impl TryFrom<ParseCmdOutput> for CmdInfos {
    type Error = conversion::Error;

    #[inline]
    fn try_from(cmd: ParseCmdOutput) -> Result<Self, Self::Error> {
        let ParseCmdOutput {
            addr,
            args,
            bang,
            cmd,
            count,
            magic,
            mods,
            nargs,
            nextcmd,
            range,
            reg,
            ..
        } = cmd;

        #[inline]
        fn deserialize<T>(
            obj: impl Into<Object>,
        ) -> Result<T, conversion::Error>
        where
            T: serde::de::DeserializeOwned,
        {
            T::deserialize(Deserializer::new(obj.into())).map_err(Into::into)
        }

        Ok(Self {
            addr: utils::none_literal_is_none(Deserializer::new(addr.into()))?,
            args: deserialize(args)?,
            bang: deserialize(bang)?,
            cmd: deserialize(cmd)?,
            count: utils::minus_one_is_none(Deserializer::new(count.into()))?,
            magic: deserialize(magic)?,
            mods: deserialize(mods)?,
            nargs: deserialize(nargs)?,
            nextcmd: utils::empty_string_is_none(Deserializer::new(
                nextcmd.into(),
            ))?,
            range: deserialize(range)?,
            reg: utils::char_from_string(Deserializer::new(reg.into()))?,
        })
    }
}
