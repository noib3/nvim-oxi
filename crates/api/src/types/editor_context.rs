use serde::Deserialize;
use types::{
    Array,
    Dictionary,
    Object,
    conversion::{self, FromObject},
    serde::Deserializer,
};

#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize)]
pub struct EditorContext {
    #[serde(default, rename = "bufs")]
    pub bufferlist: Vec<types::String>,

    #[serde(default, rename = "gvars")]
    pub global_vars: Vec<types::String>,

    #[serde(default, rename = "funcs")]
    pub global_and_script_local_funcs: Vec<types::String>,

    #[serde(default, rename = "jumps")]
    pub jumplist: Vec<types::String>,

    #[serde(default, rename = "regs")]
    pub registers: Vec<types::String>,

    #[serde(default, rename = "sfuncs")]
    pub script_local_funcs: Vec<types::String>,
}

impl EditorContext {
    #[inline(always)]
    pub fn builder() -> EditorContextBuilder {
        EditorContextBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct EditorContextBuilder(EditorContext);

impl EditorContextBuilder {
    #[inline]
    pub fn bufferlist<Line, Lines>(&mut self, lines: Lines) -> &mut Self
    where
        Lines: IntoIterator<Item = Line>,
        Line: Into<types::String>,
    {
        self.0.bufferlist = lines.into_iter().map(Into::into).collect();
        self
    }

    #[inline]
    pub fn global_vars<Line, Lines>(&mut self, lines: Lines) -> &mut Self
    where
        Lines: IntoIterator<Item = Line>,
        Line: Into<types::String>,
    {
        self.0.global_vars = lines.into_iter().map(Into::into).collect();
        self
    }

    #[inline]
    pub fn global_and_script_local_funcs<Line, Lines>(
        &mut self,
        lines: Lines,
    ) -> &mut Self
    where
        Lines: IntoIterator<Item = Line>,
        Line: Into<types::String>,
    {
        self.0.global_and_script_local_funcs =
            lines.into_iter().map(Into::into).collect();
        self
    }

    #[inline]
    pub fn jumplist<Line, Lines>(&mut self, lines: Lines) -> &mut Self
    where
        Lines: IntoIterator<Item = Line>,
        Line: Into<types::String>,
    {
        self.0.jumplist = lines.into_iter().map(Into::into).collect();
        self
    }

    #[inline]
    pub fn registers<Line, Lines>(&mut self, lines: Lines) -> &mut Self
    where
        Lines: IntoIterator<Item = Line>,
        Line: Into<types::String>,
    {
        self.0.registers = lines.into_iter().map(Into::into).collect();
        self
    }

    #[inline]
    pub fn script_local_funcs<Line, Lines>(
        &mut self,
        lines: Lines,
    ) -> &mut Self
    where
        Lines: IntoIterator<Item = Line>,
        Line: Into<types::String>,
    {
        self.0.script_local_funcs =
            lines.into_iter().map(Into::into).collect();
        self
    }

    #[inline]
    pub fn build(&mut self) -> EditorContext {
        std::mem::take(&mut self.0)
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
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
