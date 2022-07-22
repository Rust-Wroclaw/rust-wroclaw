use std::fmt;

use serde::de::Visitor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Data(String);

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}

impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(deserializer.deserialize_string(StringVisitor)?))
    }
}

#[test]
fn serde_json_from_str_fails() {
    let s = "\"asdf\"";

    let deserialized: Data = serde_json::from_str(s).unwrap();

    println!("{deserialized:?}");
}

#[test]
fn serde_json_from_value_passes() {
    let value = serde_json::Value::String("asdf".to_string());

    let deserialized: Data = serde_json::from_value(value).unwrap();

    println!("{deserialized:?}");
}
