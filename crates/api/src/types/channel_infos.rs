use serde::Deserialize;
use types::{
    Object,
    conversion::{self, FromObject},
    serde::Deserializer,
};

use super::ClientInfos;
use crate::Buffer;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct ChannelInfos {
    /// Job arguments list.
    pub argv: Option<Vec<String>>,

    /// Buffer with connected terminal instance. Only present when `mode` is
    /// `ChannelMode::Terminal`.
    pub buffer: Option<Buffer>,

    /// Info about the client on the other side of an RPC channel. Only present
    /// when `mode` is `ChannelMode::Rpc`.
    pub client: Option<ClientInfos>,

    /// Channel id.
    pub id: u32,

    /// How data receiveed on the channel is interpeted.
    pub mode: ChannelMode,

    /// Name of a pseudoterminal. On a POSIX system this is a device path like
    /// `/dev/pts/1`. If the name is unknown, the key will still be present if
    /// a pty is used (e.g. for `ConPTY` on Windows).
    pub pty: Option<String>,

    /// Stream underlying the channel.
    pub stream: ChannelStream,
}

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChannelStream {
    Job,
    Socket,
    StdErr,
    StdIo,
}

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChannelMode {
    Bytes,
    Rpc,
    Terminal,
}

impl FromObject for ChannelInfos {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
