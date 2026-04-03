#[cfg(feature = "neovim-0-12")] // on 0.12 and Nightly.
use std::fmt;

#[cfg(not(feature = "neovim-0-12"))] // On 0.11 Only
use serde::Serialize;
#[cfg(feature = "neovim-0-12")] // on 0.12 and Nightly.
use serde::{Deserialize, Serialize, de};
#[cfg(feature = "neovim-0-12")] // on 0.12 and Nightly.
use types::{
    Function,
    Object,
    conversion::{self, FromObject, ToObject},
    serde::{Deserializer, Serializer},
};
#[cfg(not(feature = "neovim-0-12"))] // On 0.11 Only
use types::{
    Function,
    Object,
    conversion::{self, ToObject},
    serde::Serializer,
};

type CustomListFunc = Function<(String, String, usize), Vec<String>>;

/// See `:h command-complete` for details.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandComplete {
    Arglist,
    Augroup,
    Buffer,
    Breakpoint,
    Color,
    Command,
    Compiler,
    #[serde(rename = "custom")]
    CustomFromCompleteArg,
    #[serde(rename = "customlist")]
    CustomlistFromCompleteArg,
    DiffBuffer,
    Dir,
    DirInPath,
    Environment,
    Event,
    Expression,
    File,
    FileInPath,
    Filetype,
    Function,
    Help,
    Highlight,
    History,
    Keymap,
    Locale,
    Lua,
    Mapclear,
    Mapping,
    Menu,
    Messages,
    Option,
    Packadd,
    #[cfg(feature = "neovim-0-12")] // On 0.12 and Nightly
    Retab,
    Runtime,
    Scriptnames,
    Shellcmd,
    Shellcmdline,
    Sign,
    Syntax,
    Syntime,
    Tag,
    TagListfiles,
    User,
    Var,

    /// See `:h command-completion-customlist` for details.
    CustomList(CustomListFunc),
}

impl ToObject for CommandComplete {
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

#[cfg(feature = "neovim-0-12")] // on 0.12 and Nightly.
impl FromObject for CommandComplete {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

#[cfg(feature = "neovim-0-12")] // on 0.12 and Nightly.
impl<'de> de::Deserialize<'de> for CommandComplete {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct CommandCompleteVisitor;

        impl de::Visitor<'_> for CommandCompleteVisitor {
            type Value = CommandComplete;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(
                    "string or function (see `:help command-completion`)",
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "arglist" => return Ok(Self::Value::Arglist),
                    "augroup" => return Ok(Self::Value::Augroup),
                    "buffer" => return Ok(Self::Value::Buffer),
                    "breakpoint" => return Ok(Self::Value::Breakpoint),
                    "color" => return Ok(Self::Value::Color),
                    "command" => return Ok(Self::Value::Command),
                    "compiler" => return Ok(Self::Value::Compiler),
                    "custom" => return Ok(Self::Value::CustomFromCompleteArg),
                    "customlist" => {
                        return Ok(Self::Value::CustomlistFromCompleteArg);
                    },
                    "diff_buffer" => return Ok(Self::Value::DiffBuffer),
                    "dir" => return Ok(Self::Value::Dir),
                    "dir_in_path" => return Ok(Self::Value::DirInPath),
                    "environment" => return Ok(Self::Value::Environment),
                    "event" => return Ok(Self::Value::Event),
                    "expression" => return Ok(Self::Value::Expression),
                    "file" => return Ok(Self::Value::File),
                    "file_in_path" => return Ok(Self::Value::FileInPath),
                    "filetype" => return Ok(Self::Value::Filetype),
                    "function" => return Ok(Self::Value::Function),
                    "help" => return Ok(Self::Value::Help),
                    "highlight" => return Ok(Self::Value::Highlight),
                    "history" => return Ok(Self::Value::History),
                    "keymap" => return Ok(Self::Value::Keymap),
                    "locale" => return Ok(Self::Value::Locale),
                    "lua" => return Ok(Self::Value::Lua),
                    "mapclear" => return Ok(Self::Value::Mapclear),
                    "mapping" => return Ok(Self::Value::Mapping),
                    "menu" => return Ok(Self::Value::Menu),
                    "messages" => return Ok(Self::Value::Messages),
                    "option" => return Ok(Self::Value::Option),
                    "packadd" => return Ok(Self::Value::Packadd),
                    #[cfg(feature = "neovim-0-12")] // On 0.12 and Nightly
                    "retab" => return Ok(Self::Value::Retab),
                    "runtime" => return Ok(Self::Value::Runtime),
                    "scriptnames" => return Ok(Self::Value::Scriptnames),
                    "shellcmd" => return Ok(Self::Value::Shellcmd),
                    "shellcmdline" => return Ok(Self::Value::Shellcmdline),
                    "sign" => return Ok(Self::Value::Sign),
                    "syntax" => return Ok(Self::Value::Syntax),
                    "syntime" => return Ok(Self::Value::Syntime),
                    "tag" => return Ok(Self::Value::Tag),
                    "tag_listfiles" => return Ok(Self::Value::TagListfiles),
                    "user" => return Ok(Self::Value::User),
                    "var" => return Ok(Self::Value::Var),

                    _ => {},
                };

                Err(E::invalid_value(
                    de::Unexpected::Str(v),
                    &"completion type (see `:help command-completion`)",
                ))
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let lua_ref = Object::from_luaref(v as i32);
                if let Ok(func) = CustomListFunc::from_object(lua_ref) {
                    return Ok(Self::Value::CustomList(func));
                }
                Err(E::invalid_value(
                    de::Unexpected::Float(v as f64),
                    &"custom_list like completion function",
                ))
            }
        }

        deserializer.deserialize_str(CommandCompleteVisitor)
    }
}
