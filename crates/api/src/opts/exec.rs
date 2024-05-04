/// Option passed to [`exec2()`][crate::exec2].
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct ExecOpts {
    /// Whether to capture and return all (non-error, non-shell |:!|) output.
    #[builder(argtype = "bool")]
    output: types::Boolean,
}
