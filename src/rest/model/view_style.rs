use serde::{de, ser};

use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewStyle {
    List,
    Board,
}

impl FromStr for ViewStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "list" => Ok(ViewStyle::List),
            "board" => Ok(ViewStyle::Board),
            _ => Err(format!("Invalid view style: {}", s)),
        }
    }
}

impl ViewStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            ViewStyle::List => "list",
            ViewStyle::Board => "board",
        }
    }
}

impl ser::Serialize for ViewStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> de::Deserialize<'de> for ViewStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ViewStyleVisitor;

        impl<'de> de::Visitor<'de> for ViewStyleVisitor {
            type Value = ViewStyle;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a view style")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                ViewStyle::from_str(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(ViewStyleVisitor)
    }
}

impl Default for ViewStyle {
    fn default() -> Self {
        ViewStyle::List
    }
}
