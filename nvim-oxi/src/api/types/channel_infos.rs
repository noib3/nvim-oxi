use nvim_types::Object;
use serde::Deserialize;

use super::ClientInfos;
use crate::api::Buffer;
use crate::object::{self, FromObject};

/// Object returned from a call to `crate::api::get_chan_info`.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct ChannelInfos {
    /// Channel id.
    pub id: u32,

    /// Job arguments list.
    pub argv: Option<Vec<String>>,

    /// Stream underlying the channel.
    pub stream: ChannelStream,

    /// How data receiveed on the channel is interpeted.
    pub mode: ChannelMode,

    /// Name of a pseudoterminal. On a POSIX system this is a device path like
    /// `/dev/pts/1`. If the name is unknown, the key will still be present if
    /// a pty is used (e.g. for `ConPTY` on Windows).
    pub pty: Option<String>,

    /// Buffer with connected terminal instance. Only present when `mode` is
    /// `ChannelMode::Terminal`.
    pub buffer: Option<Buffer>,

    /// Info about the client on the other side of an RPC channel. Only present
    /// when `mode` is `ChannelMode::Rpc`.
    pub client: Option<ClientInfos>,
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
    fn from_obj(obj: Object) -> crate::Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
