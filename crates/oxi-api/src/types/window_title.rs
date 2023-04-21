use oxi_types::{Array, Object, String};
use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum WindowTitle {
    SimpleString(String),
    ListOfText(Vec<(String, TitleHighlight)>),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum TitleHighlight {
    SimpleString(String),
    ListOfString(Vec<String>),
}

impl From<&WindowTitle> for Object {
    fn from(title: &WindowTitle) -> Self {
        match title {
            WindowTitle::SimpleString(value) => value.clone().into(),
            WindowTitle::ListOfText(list) => list
                .iter()
                .map(|(txt, hl)| {
                    Array::from_iter(
                        [txt.clone().into(), hl.into()] as [Object; 2]
                    )
                })
                .collect::<Array>()
                .into(),
        }
    }
}

impl From<&TitleHighlight> for Object {
    fn from(hl: &TitleHighlight) -> Self {
        match hl {
            TitleHighlight::SimpleString(s) => s.clone().into(),
            TitleHighlight::ListOfString(list) => {
                list.iter().cloned().collect::<Array>().into()
            },
        }
    }
}

impl From<String> for TitleHighlight {
    fn from(value: String) -> Self {
        Self::SimpleString(value)
    }
}

impl From<Vec<String>> for TitleHighlight {
    fn from(value: Vec<String>) -> Self {
        Self::ListOfString(value.into_iter().collect())
    }
}
