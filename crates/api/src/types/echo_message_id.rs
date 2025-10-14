/// The message ID returned by [`echo()`](crate::echo).
#[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EchoMessageId {
    String(types::String),
    Int(types::Integer),
}

impl From<EchoMessageId> for types::Object {
    fn from(id: EchoMessageId) -> Self {
        match id {
            EchoMessageId::String(str) => str.into(),
            EchoMessageId::Int(int) => int.into(),
        }
    }
}

impl crate::StringOrInt for EchoMessageId {
    fn to_object(self) -> types::Object {
        self.into()
    }
}
