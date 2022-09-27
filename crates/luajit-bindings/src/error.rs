use thiserror::Error as ThisError;

#[derive(Clone, Debug, Eq, PartialEq, ThisError)]
pub enum Error {
    #[error(
        "Value of type {ty} couldn't be popped from the stack{}.",
        message.as_ref().map(|msg| format!(": {msg}")).unwrap_or_default()
    )]
    PopError { ty: &'static str, message: Option<String> },

    #[error(
        "Value of type {ty} couldn't be pushed on the stack{}.",
        message.as_ref().map(|msg| format!(": {msg}")).unwrap_or_default()
    )]
    PushError { ty: &'static str, message: Option<String> },

    #[error("Lua runtime error: {0}")]
    RuntimeError(String),

    #[error("Lua memory error: {0}")]
    MemoryError(String),
}

impl Error {
    pub fn pop_error<M: Into<String>>(
        ty: &'static str,
        message: Option<M>,
    ) -> Self {
        Self::PopError { ty, message: message.map(Into::into) }
    }

    pub fn push_error<M: Into<String>>(
        ty: &'static str,
        message: Option<M>,
    ) -> Self {
        Self::PushError { ty, message: message.map(Into::into) }
    }

    pub fn push_ciao<T, E: std::error::Error>(err: E) -> Self {
        Self::PushError {
            ty: std::any::type_name::<T>(),
            message: Some(err.to_string()),
        }
    }
}
