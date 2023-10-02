use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(untagged)]
pub enum OneOrMore<T> {
    One(T),
    List(Vec<T>),
}

impl<T> From<T> for OneOrMore<T> {
    fn from(one: T) -> Self {
        OneOrMore::One(one)
    }
}

impl<T> From<Vec<T>> for OneOrMore<T> {
    fn from(vec: Vec<T>) -> Self {
        OneOrMore::List(vec)
    }
}

impl From<&str> for OneOrMore<String> {
    fn from(s: &str) -> Self {
        OneOrMore::One(s.to_owned())
    }
}
