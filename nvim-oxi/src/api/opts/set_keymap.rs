use derive_builder::Builder;
use nvim_types::{self as nvim, NonOwning, Object};

use crate::lua::LuaFun;

/// Options passed to `Buffer::set_keymap`.
#[derive(Clone, Debug, Default, PartialEq, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct SetKeymapOpts {
    #[builder(setter(custom))]
    callback: Object,

    #[builder(setter(custom))]
    desc: Object,

    #[builder(setter(strip_option))]
    expr: Option<bool>,

    #[builder(setter(strip_option))]
    noremap: Option<bool>,

    #[builder(setter(strip_option))]
    nowait: Option<bool>,

    #[builder(setter(strip_option))]
    script: Option<bool>,

    #[builder(setter(strip_option))]
    silent: Option<bool>,

    #[builder(setter(strip_option))]
    unique: Option<bool>,
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
        self.callback = Some(LuaFun::from_fn_mut(fun).into());
        self
    }

    pub fn desc(&mut self, desc: impl Into<nvim::String>) -> &mut Self {
        self.desc = Some(desc.into().into());
        self
    }

    pub fn build(&mut self) -> SetKeymapOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_keymap<'a> {
    desc: NonOwning<'a, Object>,
    expr: Object,
    script: Object,
    silent: Object,
    unique: Object,
    nowait: Object,
    noremap: Object,
    callback: NonOwning<'a, Object>,
}

impl<'a> From<&'a SetKeymapOpts> for KeyDict_keymap<'a> {
    fn from(opts: &'a SetKeymapOpts) -> Self {
        Self {
            desc: opts.desc.non_owning(),
            expr: opts.expr.into(),
            script: opts.script.into(),
            silent: opts.silent.into(),
            unique: opts.unique.into(),
            nowait: opts.nowait.into(),
            noremap: opts.noremap.into(),
            callback: opts.callback.non_owning(),
        }
    }
}
