extern crate serde_yaml;
pub mod sigma {
    use super::custom_deserialize::{deserialize_level, deserialize_status};
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::Read;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct LogSource {
        service: Option<String>,
        category: Option<String>,
        product: Option<String>,
    }

    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub enum SigmaStatus {
        Test,
        Stable,
        // TODO complete
    }

    impl Serialize for SigmaStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                SigmaStatus::Test => serializer.serialize_str("test"),
                SigmaStatus::Stable => serializer.serialize_str("stable"),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub enum SigmaLevel {
        Critical,
        High,
        Medium,
        Low,
        Informational,
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct SigmaDetecton {
        condition: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SigmaRule {
        title: String,
        #[serde(deserialize_with = "deserialize_status")]
        status: SigmaStatus,
        description: String,
        tags: Vec<String>,
        #[serde(deserialize_with = "deserialize_level")]
        level: SigmaLevel,
        logsource: LogSource,
        detection: SigmaDetecton,
    }

    impl SigmaRule {
        pub fn parse_rule_from_file(path: String) -> SigmaRule {
            // Open the YAML file
            let mut file = File::open(path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            serde_yaml::from_str::<SigmaRule>(&contents).unwrap()
        }
    }

    #[test]
    #[should_panic]
    fn invalid_path() {
        SigmaRule::parse_rule_from_file("azeaze".to_string());
    }

    #[test]
    fn parse_from_file() {
        let rule: SigmaRule = SigmaRule::parse_rule_from_file("tests/test_rule.yml".to_string());

        assert_eq!(rule.title, "test rule".to_string());
        assert_eq!(rule.status, SigmaStatus::Test);
        assert_eq!(
            rule.description,
            "this rule is for test purpose".to_string()
        );
        assert_eq!(
            rule.tags,
            vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()]
        );
        assert_eq!(rule.level, SigmaLevel::Medium);
        assert_eq!(
            rule.logsource,
            LogSource {
                product: Some("windows".to_string()),
                category: Some("dns_query".to_string()),
                service: None
            }
        );
    }
}

pub mod custom_deserialize {
    use super::sigma::{SigmaLevel, SigmaStatus};
    use serde::de;

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
    pub fn deserialize_level<'de, D>(deserializer: D) -> Result<SigmaLevel, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = SigmaLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a status")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "informational" => Ok(SigmaLevel::Informational),
                    "low" => Ok(SigmaLevel::Low),
                    "medium" => Ok(SigmaLevel::Medium),
                    "high" => Ok(SigmaLevel::High),
                    "critical" => Ok(SigmaLevel::Critical),
                    _ => Err(de::Error::custom(format!("Invalid status: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
