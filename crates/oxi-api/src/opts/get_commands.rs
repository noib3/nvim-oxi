use oxi_types::Object;

/// Options passed to [`Buffer::get_commands()`](crate::Buffer::get_commands)
/// and [`get_commands()`](crate::get_commands).
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct GetCommandsOpts {
    builtin: Object,
}

impl GetCommandsOpts {
    #[inline(always)]
    pub fn builder() -> GetCommandsOptsBuilder {
        GetCommandsOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct GetCommandsOptsBuilder(GetCommandsOpts);

impl GetCommandsOptsBuilder {
    #[inline]
    pub fn builtin(&mut self, builtin: bool) -> &mut Self {
        self.0.builtin = builtin.into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> GetCommandsOpts {
        std::mem::take(&mut self.0)
    }
}
