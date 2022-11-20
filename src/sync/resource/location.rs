use serde::{de, ser, Serialize};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Location {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

//The location object is specific, as it's not an object, but an ordered array.
// 0: name, 1: latitude, 2: longitude
impl ser::Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        #[derive(Serialize)]
        #[serde(rename = "T")]
        struct Helper<'a>(&'a String, f64, f64);

        let helper = Helper(&self.name, self.latitude, self.longitude);
        helper.serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for Location {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct LocationVisitor;

        impl<'de> de::Visitor<'de> for LocationVisitor {
            type Value = Location;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a location object")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let name = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let latitude = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let longitude = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                Ok(Location {
                    name,
                    latitude,
                    longitude,
                })
            }
        }

        deserializer.deserialize_seq(LocationVisitor)
    }
}
