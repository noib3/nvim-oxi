use serde::Deserialize;

/// Split modifier passed to the `split` key of `CommandModifiers`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SplitModifier {
    /// See `:h `:aboveleft`` for more infos.
    AboveLeft,

    /// See `:h `:belowright`` for more infos.
    BelowRight,

    /// See `:h `:topleft`` for more infos.
    TopLeft,

    /// See `:h `:botright`` for more infos.
    BotRight,
}
