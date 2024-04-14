use serde::Deserialize;
use types::{
    conversion::{self, FromObject, ToObject},
    serde::Deserializer,
    Array,
    Object,
};
#[cfg(feature = "neovim-nightly")]
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
    mask: u64,

    /// 1st in the mask.
    cmd: NvimString,

    /// 10th in the mask.
    range: Array,

    /// 7th in the mask.
    count: Integer,

    /// 2nd in the mask.
    reg: NvimString,

    /// 3rd in the mask.
    bang: Boolean,

    /// 6th in the mask.
    args: Array,

    /// 8th in the mask.
    magic: Dictionary,

    /// 5th in the mask.
    mods: Dictionary,

    /// 9th in the mask.
    nargs: Object,

    /// 4th in the mask.
    addr: Object,

    /// 11th in the mask.
    nextcmd: Object,
}

impl From<&CmdInfos> for KeyDict_cmd {
    #[inline]
    fn from(infos: &CmdInfos) -> Self {
        #[cfg(not(feature = "neovim-nightly"))]
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
        #[cfg(feature = "neovim-nightly")]
        {
            let mut mask = 0;

            let cmd = if let Some(cmd) = infos.cmd.as_deref() {
                mask |= 0b11;
                NvimString::from(cmd)
            } else {
                NvimString::default()
            };

            let range = if let Some(range) = infos.range {
                mask |= 0b10000000001;
                Array::from(range)
            } else {
                Array::default()
            };

            let count = if let Some(count) = infos.count {
                mask |= 0b10000001;
                count as Integer
            } else {
                Integer::default()
            };

            let reg = if let Some(reg) = infos.reg {
                mask |= 0b101;
                reg.into()
            } else {
                NvimString::default()
            };

            let bang = if let Some(bang) = infos.bang {
                mask |= 0b1001;
                bang
            } else {
                Boolean::default()
            };

            let args = if !infos.args.is_empty() {
                mask |= 0b1000001;
                Array::from_iter(infos.args.clone())
            } else {
                Array::default()
            };

            let magic = if let Some(magic) = infos.magic {
                mask |= 0b100000001;
                Dictionary::from(magic)
            } else {
                Dictionary::default()
            };

            let mods = if let Some(mods) = infos.mods {
                mask |= 0b100001;
                Dictionary::from(mods)
            } else {
                Dictionary::default()
            };

            let nargs = if let Some(nargs) = infos.nargs {
                mask |= 0b1000000001;
                nargs.to_object().unwrap()
            } else {
                Object::default()
            };

            let addr = if let Some(addr) = infos.addr {
                mask |= 0b10001;
                addr.to_object().unwrap()
            } else {
                Object::default()
            };

            let nextcmd = if let Some(nextcmd) = infos.nextcmd.as_deref() {
                mask |= 0b100000000001;
                NvimString::from(nextcmd).into()
            } else {
                Object::default()
            };

            Self {
                mask,
                cmd,
                reg,
                bang,
                addr,
                mods,
                args,
                count,
                magic,
                nargs,
                range,
                nextcmd,
            }
        }
    }
}

#[cfg(feature = "neovim-nightly")]
impl TryFrom<KeyDict_cmd> for CmdInfos {
    type Error = conversion::Error;

    #[inline]
    fn try_from(cmd: KeyDict_cmd) -> Result<Self, Self::Error> {
        let KeyDict_cmd {
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
            addr: utils::none_literal_is_none(Deserializer::new(addr))?,
            args: deserialize(args)?,
            bang: deserialize(bang)?,
            cmd: deserialize(cmd)?,
            count: utils::minus_one_is_none(Deserializer::new(count.into()))?,
            magic: deserialize(magic)?,
            mods: deserialize(mods)?,
            nargs: deserialize(nargs)?,
            nextcmd: utils::empty_string_is_none(Deserializer::new(nextcmd))?,
            range: deserialize(range)?,
            reg: utils::char_from_string(Deserializer::new(reg.into()))?,
        })
    }
}
