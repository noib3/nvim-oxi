use derive_builder::Builder;
use nvim_types::{
    dictionary::Dictionary,
    string::String as NvimString,
    LuaRef,
};

use crate::lua::LuaFun;
use crate::object::ToObject;

#[derive(Clone, Debug, Default, Builder)]
#[builder(default)]
pub struct SetKeymapOpts {
    #[builder(setter(custom))]
    callback: Option<LuaRef>,

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
    pub fn builder() -> SetKeymapOpts {
        SetKeymapOpts::default()
    }
}

impl SetKeymapOptsBuilder {
    pub fn callback<F>(&mut self, fun: F) -> &mut Self
    where
        F: FnMut(()) -> crate::Result<()> + 'static,
    {
        self.callback = Some(Some(LuaFun::from_fn_mut(fun).0));
        self
    }
}

impl From<SetKeymapOpts> for Dictionary {
    fn from(opts: SetKeymapOpts) -> Self {
        Self::from_iter([
            ("callback", opts.callback.to_obj()),
            ("desc", opts.desc.to_obj()),
            ("expr", opts.expr.to_obj()),
            ("noremap", opts.noremap.to_obj()),
            ("nowait", opts.nowait.to_obj()),
            ("script", opts.script.to_obj()),
            ("silent", opts.silent.to_obj()),
            ("unique", opts.unique.to_obj()),
        ])
    }
}

impl<'a> From<&'a SetKeymapOpts> for Dictionary {
    fn from(opts: &SetKeymapOpts) -> Self {
        opts.clone().into()
    }
}
