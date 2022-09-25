use thiserror::Error as ThisError;

#[derive(Clone, Debug, Eq, PartialEq, ThisError)]
pub enum Error {
    #[error("Lua runtime error: {0}")]
    RuntimeError(String),

    #[error("Lua memory error: {0}")]
    MemoryError(String),
}
