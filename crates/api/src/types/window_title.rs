use serde::Deserialize;
use types::{Array, Object, String};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(untagged)]
pub enum WindowTitle {
    SimpleString(String),

    /// A list of `(text, highlight)` tuples, where the `highlight` is
    /// optional.
    #[serde(deserialize_with = "deserialize_tuple")]
    ListOfText(Vec<(String, Option<String>)>),
}

impl From<&WindowTitle> for Object {
    fn from(title: &WindowTitle) -> Self {
        match title {
            WindowTitle::SimpleString(value) => value.clone().into(),
            WindowTitle::ListOfText(list) => list
                .iter()
                .map(|(text, maybe_hl)| {
                    let txt: Object = text.clone().into();
                    let hl = maybe_hl
                        .as_ref()
                        .map(|hl| hl.clone().into())
                        .unwrap_or_default();
                    Array::from_iter([txt, hl])
                })
                .collect::<Array>()
                .into(),
        }
    }
}

fn deserialize_tuple<'de, D>(
    deserializer: D,
) -> Result<Vec<(String, Option<String>)>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(Vec::<Vec<String>>::deserialize(deserializer)?
        .into_iter()
        .map(|tuple| {
            let mut iter = tuple.into_iter();
            let text = iter.next().expect("text is always present");
            let maybe_hl = iter.next();
            debug_assert!(iter.next().is_none());
            (text, maybe_hl)
        })
        .collect())
}
