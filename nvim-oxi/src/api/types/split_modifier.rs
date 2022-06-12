use serde::Deserialize;

/// Split modifier passed to the `split` key of `CommandModifiers`.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SplitModifier {
    /// See `:h `:aboveleft`` for more infos.
    AboveLeft,

    /// See `:h `:aboveleft`` for more infos.
    BelowRight,

    /// See `:h `:aboveleft`` for more infos.
    TopLeft,

    /// See `:h `:aboveleft`` for more infos.
    BotRight,
}
