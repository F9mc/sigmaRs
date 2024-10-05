extern crate serde_json;
extern crate serde_yaml;
pub mod sigma {
    use super::custom_deserialize::{deserialize_level, deserialize_status};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::HashMap;
    use std::fmt;
    use std::fs::File;
    use std::io::Read;
    use walkdir::WalkDir;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct LogSource {
        service: Option<String>,
        category: Option<String>,
        product: Option<String>,
    }

    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub enum SigmaStatus {
        Stable,
        Test,
        Experimental,
        Deprecated,
        Unsupported,
    }

    impl Serialize for SigmaStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                SigmaStatus::Test => serializer.serialize_str("test"),
                SigmaStatus::Stable => serializer.serialize_str("stable"),
                SigmaStatus::Experimental => serializer.serialize_str("experimental"),
                SigmaStatus::Deprecated => serializer.serialize_str("deprecated"),
                SigmaStatus::Unsupported => serializer.serialize_str("unsupported"),
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

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct SigmaDetecton {
        condition: String,
        keywords: Option<Vec<String>>,
        #[serde(flatten)]
        selections: Option<HashMap<String, Value>>,
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

    #[derive(Debug, PartialEq, Eq)]
    pub enum ParsingError {
        InvalidFile,
        InvalidAttribute,
    }

    impl SigmaRule {
        pub fn parse_rule_from_file(path: String) -> Result<SigmaRule, ParsingError> {
            // Open the YAML file
            let file = File::open(path);
            let mut contents = String::new();

            match file {
                Ok(mut data) => {
                    let _ = data.read_to_string(&mut contents);
                }
                Err(_) => return Err(ParsingError::InvalidAttribute),
            }

            match serde_yaml::from_str::<SigmaRule>(&contents) {
                Ok(val) => Ok(val),
                Err(_) => Err(ParsingError::InvalidAttribute),
            }
        }

        pub fn load_rule_from_folder(path: String) -> Vec<SigmaRule> {
            let mut vec = Vec::new();
            for entry in WalkDir::new(&path).into_iter() {
                match entry {
                    Ok(entry) => {
                        match SigmaRule::parse_rule_from_file(
                            entry.path().to_string_lossy().into_owned(),
                        ) {
                            Ok(rule) => vec.push(rule),
                            Err(ParsingError::InvalidAttribute) => {
                                println!("Error parsing the file {path}")
                            }
                            Err(ParsingError::InvalidFile) => {
                                println!("Error reading the file")
                            }
                            Err(_) => println!("Unknwoned error"),
                        };
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            vec
        }
    }

    impl fmt::Display for SigmaRule {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.title)
        }
    }

    #[test]
    #[should_panic]
    fn invalid_path() {
        SigmaRule::parse_rule_from_file("azeaze".to_string()).unwrap();
    }

    #[test]
    fn parse_from_file() {
        let rule: SigmaRule =
            SigmaRule::parse_rule_from_file("tests/test_rule.yml".to_string()).unwrap();

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

        let mut sections = HashMap::new();
        sections.insert(
            "selection1".to_string(),
            json!({"process": "toto", "file": "tata"}),
        );
        sections.insert("selection2".to_string(), json!({"image": ["tutu", "tyty"]}));
        let detection = SigmaDetecton {
            condition: "selection1 or selection1 or keywords".to_string(),
            keywords: Some(vec!["titi".to_string(), "tete".to_string()]),
            selections: Some(sections),
        };
        assert_eq!(rule.detection, detection);
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
                    "experimental" => Ok(SigmaStatus::Experimental),
                    "deprecated" => Ok(SigmaStatus::Deprecated),
                    "unsupported" => Ok(SigmaStatus::Unsupported),
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
