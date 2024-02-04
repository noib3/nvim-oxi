use serde::{ser, Serialize};
use types::{conversion::FromObject, serde::Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum RegisterType {
    #[serde(serialize_with = "serialize_blockwise")]
    BlockwiseVisual(Option<usize>),

    #[serde(rename = "c")]
    Charwise,

    #[serde(rename = "l")]
    Linewise,

    #[serde(rename = "")]
    Guess,
}

fn serialize_blockwise<S>(
    width: &Option<usize>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    serializer.serialize_str(
        &(match width {
            Some(n) => format!("b{n}"),
            None => "b".to_owned(),
        }),
    )
}

impl From<RegisterType> for types::String {
    fn from(reg_type: RegisterType) -> Self {
        let obj = reg_type
            .serialize(Serializer::new())
            .expect("`RegisterType` is serializable");

        Self::from_object(obj)
            .expect("`RegisterType` is serialized into a string")
    }
}
