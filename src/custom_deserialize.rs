pub mod custom_deserialize {
    use serde::{de, Deserialize};

    pub fn deserialize_status<'de, D>(deserializer: D) -> Result<SigmaStatus, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = SigmaStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a status")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "test" => Ok(SigmaStatus::Test),
                    "stable" => Ok(SigmaStatus::Stable),
                    _ => Err(de::Error::custom(format!("Invalid status: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
