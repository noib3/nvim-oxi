//! Utility functions for deserializing values coming from Neovim.

use serde::de::{self, Deserialize, Deserializer, IntoDeserializer};

pub(crate) fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),

        other => Err(de::Error::invalid_value(
            de::Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

pub(crate) fn char_from_string<'de, D>(
    deserializer: D,
) -> Result<Option<char>, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    match str.len() {
        0 => Ok(None),
        1 => Ok(str.chars().next()),
        other => Err(de::Error::invalid_length(
            other,
            &"empty string or string with a single character",
        )),
    }
}

pub(crate) fn empty_string_is_none<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let str = Option::<String>::deserialize(deserializer)?;

    match str {
        None => Ok(None),
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

pub(crate) fn minus_one_is_none<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let num = i64::deserialize(deserializer)?;

    match num {
        -1 => Ok(None),
        n => T::deserialize(n.into_deserializer()).map(Some),
    }
}

pub(crate) fn none_literal_is_none<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: de::Deserializer<'de>,
    T: Deserialize<'de>,
{
    let str = Option::<String>::deserialize(deserializer)?;

    match str {
        None => Ok(None),
        Some(s) if s == "none" => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

pub(crate) fn zero_is_none<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let num = i64::deserialize(deserializer)?;

    match num {
        0 => Ok(None),
        n => T::deserialize(n.into_deserializer()).map(Some),
    }
}
