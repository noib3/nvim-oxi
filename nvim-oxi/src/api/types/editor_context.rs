use derive_builder::Builder;
use nvim_types::{self as nvim, Array, Dictionary, Object};
use serde::Deserialize;

use crate::object::{self, FromObject};

#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct EditorContext {
    #[builder(setter(custom))]
    #[serde(default, rename = "bufs")]
    pub bufferlist: Vec<nvim::String>,

    #[builder(setter(custom))]
    #[serde(default, rename = "gvars")]
    pub global_vars: Vec<nvim::String>,

    #[builder(setter(custom))]
    #[serde(default, rename = "funcs")]
    pub global_and_script_local_funcs: Vec<nvim::String>,

    #[builder(setter(custom))]
    #[serde(default, rename = "jumps")]
    pub jumplist: Vec<nvim::String>,

    #[builder(setter(custom))]
    #[serde(default, rename = "regs")]
    pub registers: Vec<nvim::String>,

    #[builder(setter(custom))]
    #[serde(default, rename = "sfuncs")]
    pub script_local_funcs: Vec<nvim::String>,
}

impl EditorContext {
    #[inline(always)]
    pub fn builder() -> EditorContextBuilder {
        EditorContextBuilder::default()
    }
}

macro_rules! strvec_setter {
    ($name:ident) => {
        pub fn $name<Line, Lines>(&mut self, lines: Lines) -> &mut Self
        where
            Lines: IntoIterator<Item = Line>,
            Line: Into<nvim::String>,
        {
            self.$name =
                Some(lines.into_iter().map(Into::into).collect::<Vec<_>>());
            self
        }
    };
}

impl EditorContextBuilder {
    strvec_setter!(bufferlist);
    strvec_setter!(global_vars);
    strvec_setter!(global_and_script_local_funcs);
    strvec_setter!(jumplist);
    strvec_setter!(registers);
    strvec_setter!(script_local_funcs);

    pub fn build(&mut self) -> EditorContext {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<EditorContext> for Dictionary {
    fn from(ctx: EditorContext) -> Self {
        Self::from_iter([
            ("bufferlist", Array::from_iter(ctx.bufferlist)),
            ("global_vars", Array::from_iter(ctx.global_vars)),
            (
                "global_and_script_local_funcs",
                Array::from_iter(ctx.global_and_script_local_funcs),
            ),
            ("jumplist", Array::from_iter(ctx.jumplist)),
            ("registers", Array::from_iter(ctx.registers)),
            ("script_local_funcs", Array::from_iter(ctx.script_local_funcs)),
        ])
    }
}

impl FromObject for EditorContext {
    fn from_obj(obj: Object) -> crate::Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
