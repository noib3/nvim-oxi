use core::fmt;
use std::error::Error as StdError;

use serde::{de, ser};

/// The [`Serializer`](ser::Serializer)'s error type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializeError {
    pub msg: String,
}

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl StdError for SerializeError {}

impl ser::Error for SerializeError {
    #[inline]
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self { msg: msg.to_string() }
    }
}

/// The [`Deserializer`](de::Deserializer)'s error type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeserializeError {
    Custom { msg: String },

    DuplicateField { field: &'static str },

    MissingField { field: &'static str },

    UnknownField { variant: String, expected: &'static [&'static str] },

    UnknownVariant { field: String, expected: &'static [&'static str] },
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom { msg } => write!(f, "{}", msg),
            Self::DuplicateField { field } => {
                write!(f, "duplicate field '{}'", field)
            },
            Self::MissingField { field } => {
                write!(f, "missing field '{}'", field)
            },
            Self::UnknownField { variant, expected } => {
                write!(
                    f,
                    "unknown field '{}', expected one of: {}",
                    variant,
                    expected.join(", ")
                )
            },
            Self::UnknownVariant { field, expected } => {
                write!(
                    f,
                    "unknown variant '{}', expected one of: {}",
                    field,
                    expected.join(", ")
                )
            },
        }
    }
}

impl StdError for DeserializeError {}

impl de::Error for DeserializeError {
    #[inline]
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Custom { msg: msg.to_string() }
    }

    #[inline]
    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        Self::UnknownField { variant: field.to_string(), expected }
    }

    #[inline]
    fn unknown_variant(
        field: &str,
        expected: &'static [&'static str],
    ) -> Self {
        Self::UnknownVariant { field: field.to_string(), expected }
    }

    #[inline]
    fn missing_field(field: &'static str) -> Self {
        Self::MissingField { field }
    }

    #[inline]
    fn duplicate_field(field: &'static str) -> Self {
        Self::DuplicateField { field }
    }
}
