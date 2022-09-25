use nvim_types::{self as nvim, FromObject, Serializer};
use serde::{ser, Serialize};

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

impl From<RegisterType> for nvim::String {
    fn from(reg_type: RegisterType) -> Self {
        nvim::String::from_obj(
            reg_type
                .serialize(Serializer::new())
                .expect("`RegisterType` is serializable"),
        )
        .expect("`RegisterType` is serialized into a string")
    }
}
