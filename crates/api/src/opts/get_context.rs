use crate::types::ContextType;

/// Options passed to [`get_context()`](crate::get_context).
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct GetContextOpts {
    #[builder(mask)]
    mask: u64,

    /// List of [`ContextType`]s to gather, or empty for all.
    #[builder(
        generics = "T: IntoIterator<Item = ContextType>",
        argtype = "T",
        inline = "{0}.into_iter().map(types::String::from).collect()"
    )]
    types: types::Array,
}
