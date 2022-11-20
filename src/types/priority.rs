use serde::{de, ser, Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Copy)]
#[repr(u8)]
pub enum Priority {
    #[default]
    Natural = 1,
    Medium = 2,
    Urgent = 3,
    VeryUrgent = 4,
}

impl From<u8> for Priority {
    fn from(value: u8) -> Self {
        match value {
            0 => Priority::default(),
            1 => Priority::Natural,
            2 => Priority::Medium,
            3 => Priority::Urgent,
            4 => Priority::VeryUrgent,
            _ => Priority::VeryUrgent,
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
        serializer.serialize_u8(u8::from(*self))
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
