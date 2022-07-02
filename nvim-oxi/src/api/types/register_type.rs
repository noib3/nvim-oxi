use nvim_types as nvim;
use serde::{Serialize, Serializer};

use crate::object;

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
    S: Serializer,
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
        reg_type
            .serialize(object::Serializer)
            .expect("`RegisterType` is serializable")
            .try_into()
            .expect("`RegisterType` is serialized into a string")
    }
}
