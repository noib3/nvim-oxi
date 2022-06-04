use derive_builder::Builder;
use nvim_types::{
    dictionary::Dictionary,
    object::Object,
    string::String as NvimString,
};

use crate::lua::LuaFnMut;

/// Options passed to `Buffer::set_keymap`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default)]
pub struct SetKeymapOpts {
    #[builder(setter(custom))]
    callback: Option<LuaFnMut<(), ()>>,

    #[builder(setter(into, strip_option))]
    desc: Option<NvimString>,

    expr: bool,
    noremap: bool,
    nowait: bool,
    script: bool,
    silent: bool,
    unique: bool,
}

impl SetKeymapOpts {
    #[inline(always)]
    pub fn builder() -> SetKeymapOptsBuilder {
        SetKeymapOptsBuilder::default()
    }
}

impl SetKeymapOptsBuilder {
    pub fn callback<F>(&mut self, fun: F) -> &mut Self
    where
        F: FnMut(()) -> crate::Result<()> + 'static,
    {
        self.callback = Some(Some(fun.into()));
        self
    }
}

impl From<SetKeymapOpts> for Dictionary {
    fn from(opts: SetKeymapOpts) -> Self {
        Self::from_iter([
            ("callback", Object::from(opts.callback)),
            ("desc", opts.desc.into()),
            ("expr", opts.expr.into()),
            ("noremap", opts.noremap.into()),
            ("nowait", opts.nowait.into()),
            ("script", opts.script.into()),
            ("silent", opts.silent.into()),
            ("unique", opts.unique.into()),
        ])
    }
}

impl<'a> From<&'a SetKeymapOpts> for Dictionary {
    fn from(opts: &SetKeymapOpts) -> Self {
        opts.clone().into()
    }
}
