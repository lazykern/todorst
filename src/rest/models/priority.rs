use serde::{de, ser, Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Priority {
    #[default]
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

impl From<u8> for Priority {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            _ => panic!("Unknown priority value: {}", value),
        }
    }
}

impl From<Priority> for u8 {
    fn from(value: Priority) -> Self {
        value as u8
    }
}

impl Serialize for Priority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for Priority {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Ok(Priority::from(value))
    }
}
