use std::collections::HashMap;
use std::fmt;
use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize, de};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
/// Informations about a remote RPC client.
pub struct ClientInfos {
    /// Arbitrary map of informal client properties. Suggested keys:
    ///  - "website": client homepage URL (e.g. GitHub repository);
    ///  - "license": license description ("Apache 2", "GPLv3", "MIT", …);
    ///  - "logo": URI or path to image.
    pub attributes: HashMap<String, String>,

    /// Builtin methods in the client, where map keys represent method names.
    pub methods: HashMap<String, ClientMethod>,

    /// Short name for the connected client.
    pub name: String,

    /// Advertised type of remote client.
    pub r#type: ClientType,

    /// Describes the client version.
    pub version: ClientVersion,
}

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ClientVersion {
    /// Major version
    pub major: Option<u32>,

    /// Minor version
    pub minor: Option<u32>,

    /// Patch number
    pub patch: Option<u32>,

    /// String describing a prerelease, like `"dev"` or `"beta1"`.
    pub prerelease: Option<String>,

    /// Commit hash or similar identifier of commit.
    pub commit: Option<String>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClientType {
    /// Remote client connected to Neovim.
    Remote,

    /// GUI frontend.
    Ui,

    /// Application using Neovim as a component (for example, IDE/editor
    /// implementing a vim mode).
    Embedder,

    /// Plugin host, typically started by Neovim.
    Host,

    /// Single plugin, started by Neovim.
    Plugin,
}

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct ClientMethod {
    /// Whether the method is called as a notification. If `false` or
    /// unspecified a blocking request will be used.
    pub r#async: Option<bool>,

    /// Number of arguments. Can either be a single integer or an inclusive
    /// range representive the minimum and maximum number of arguments
    /// accepted.
    pub nargs: Option<ClientMethodNArgs>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
/// Number of arguments accepted by a client method.
pub enum ClientMethodNArgs {
    Exact(u32),
    Range(RangeInclusive<u32>),
}

impl<'de> de::Deserialize<'de> for ClientMethodNArgs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ClientMethodNArgsVisitor;

        impl<'de> de::Visitor<'de> for ClientMethodNArgsVisitor {
            type Value = ClientMethodNArgs;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(
                    "either a single integer or an array of two integers",
                )
            }

            fn visit_i64<E>(self, n: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Self::Value::Exact(n as u32))
            }

            fn visit_seq<S>(
                self,
                mut visitor: S,
            ) -> Result<Self::Value, S::Error>
            where
                S: de::SeqAccess<'de>,
            {
                let len = visitor
                    .size_hint()
                    .expect("couldn't determine number of arguments");

                if len != 2 {
                    return Err(de::Error::invalid_length(
                        len,
                        &"was expecting two integers, a minimum and a maximum",
                    ));
                }

                let min = visitor.next_element::<u32>()?.unwrap();
                let max = visitor.next_element::<u32>()?.unwrap();

                Ok(Self::Value::Range(RangeInclusive::new(min, max)))
            }
        }

        deserializer.deserialize_str(ClientMethodNArgsVisitor)
    }
}
