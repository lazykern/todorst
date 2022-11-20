use serde::{de, ser};

use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    BerryRed,
    Red,
    Orange,
    Yellow,
    OliveGreen,
    LimeGreen,
    Green,
    MintGreen,
    Teal,
    SkyBlue,
    LightBlue,
    Blue,
    Grape,
    Violet,
    Lavender,
    Magenta,
    Salmon,
    Charcoal,
    Grey,
    Taupe,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "berry_red" => Ok(Color::BerryRed),
            "red" => Ok(Color::Red),
            "orange" => Ok(Color::Orange),
            "yellow" => Ok(Color::Yellow),
            "olive_green" => Ok(Color::OliveGreen),
            "lime_green" => Ok(Color::LimeGreen),
            "green" => Ok(Color::Green),
            "mint_green" => Ok(Color::MintGreen),
            "teal" => Ok(Color::Teal),
            "sky_blue" => Ok(Color::SkyBlue),
            "light_blue" => Ok(Color::LightBlue),
            "blue" => Ok(Color::Blue),
            "grape" => Ok(Color::Grape),
            "violet" => Ok(Color::Violet),
            "lavender" => Ok(Color::Lavender),
            "magenta" => Ok(Color::Magenta),
            "salmon" => Ok(Color::Salmon),
            "charcoal" => Ok(Color::Charcoal),
            "grey" => Ok(Color::Grey),
            "taupe" => Ok(Color::Taupe),
            _ => Err(format!("Invalid color: {}", s)),
        }
    }
}

impl Color {
    pub fn as_str(&self) -> &'static str {
        match self {
            Color::BerryRed => "berry_red",
            Color::Red => "red",
            Color::Orange => "orange",
            Color::Yellow => "yellow",
            Color::OliveGreen => "olive_green",
            Color::LimeGreen => "lime_green",
            Color::Green => "green",
            Color::MintGreen => "mint_green",
            Color::Teal => "teal",
            Color::SkyBlue => "sky_blue",
            Color::LightBlue => "light_blue",
            Color::Blue => "blue",
            Color::Grape => "grape",
            Color::Violet => "violet",
            Color::Lavender => "lavender",
            Color::Magenta => "magenta",
            Color::Salmon => "salmon",
            Color::Charcoal => "charcoal",
            Color::Grey => "grey",
            Color::Taupe => "taupe",
        }
    }
}

struct ColorVisitor;

impl<'de> de::Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a color in [berry_red, red, orange, yellow, olive_green, lime_green, green, mint_green, teal, sky_blue, light_blue, blue, grape, violet, lavender, magenta, salmon, charcoal, grey, taupe]")
    }

    fn visit_str<E>(self, value: &str) -> Result<Color, E>
    where
        E: de::Error,
    {
        Color::from_str(value).map_err(|e| E::custom(e))
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::Charcoal
    }
}

impl ser::Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *self {
            Color::BerryRed => serializer.serialize_str("berry_red"),
            Color::Red => serializer.serialize_str("red"),
            Color::Orange => serializer.serialize_str("orange"),
            Color::Yellow => serializer.serialize_str("yellow"),
            Color::OliveGreen => serializer.serialize_str("olive_green"),
            Color::LimeGreen => serializer.serialize_str("lime_green"),
            Color::Green => serializer.serialize_str("green"),
            Color::MintGreen => serializer.serialize_str("mint_green"),
            Color::Teal => serializer.serialize_str("teal"),
            Color::SkyBlue => serializer.serialize_str("sky_blue"),
            Color::LightBlue => serializer.serialize_str("light_blue"),
            Color::Blue => serializer.serialize_str("blue"),
            Color::Grape => serializer.serialize_str("grape"),
            Color::Violet => serializer.serialize_str("violet"),
            Color::Lavender => serializer.serialize_str("lavender"),
            Color::Magenta => serializer.serialize_str("magenta"),
            Color::Salmon => serializer.serialize_str("salmon"),
            Color::Charcoal => serializer.serialize_str("charcoal"),
            Color::Grey => serializer.serialize_str("grey"),
            Color::Taupe => serializer.serialize_str("taupe"),
        }
    }
}

impl<'de> de::Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}
