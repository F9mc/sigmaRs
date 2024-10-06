extern crate serde_json;
extern crate serde_yaml;
use crate::custom_error::ParsingError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SentinelLogSource {
    value: String,
    service: Option<String>,
    category: Option<String>,
    product: Option<String>,
}

impl SentinelLogSource {
    pub fn load_conf_file(path: String) -> Result<Vec<SentinelLogSource>, ParsingError> {
        let mut categories: Vec<SentinelLogSource> = Vec::new();

        match std::fs::read_to_string(path) {
            Ok(c) => match serde_yaml::from_str::<serde_yaml::Value>(&c) {
                Err(_) => return Err(ParsingError::InvalidAttribute),
                Ok(yaml_value) => {
                    if let Some(map) = yaml_value.as_mapping() {
                        for (_, value) in map.iter() {
                            let s = SentinelLogSource {
                                value: value["value"].as_str().unwrap_or("").to_string(),
                                service: value
                                    .get("service")
                                    .map(|v| v.as_str().unwrap_or("").to_string())
                                    .or_else(|| None),
                                category: value
                                    .get("category")
                                    .map(|v| v.as_str().unwrap_or("").to_string())
                                    .or_else(|| None),
                                product: value
                                    .get("product")
                                    .map(|v| v.as_str().unwrap_or("").to_string())
                                    .or_else(|| None),
                            };

                            categories.push(s);
                        }
                    }

                    Ok(categories)
                }
            },
            Err(_) => return Err(ParsingError::InvalidFile),
        }
    }

    pub fn load_sources(custom_path: String) -> Vec<SentinelLogSource> {
        let mut sources: Vec<SentinelLogSource> = Vec::new();

        match SentinelLogSource::load_conf_file("./src/log_source.yml".to_string()) {
            Err(_) => panic!("Cannot find default source logs"),
            Ok(res) => sources.extend(res),
        };
        match SentinelLogSource::load_conf_file(custom_path) {
            Err(_) => println!("Custom path cannot be found"),
            Ok(res) => sources.extend(res),
        };

        sources
    }

    pub fn get_sources(
        sources: &Vec<SentinelLogSource>,
        category: Option<String>,
        product: Option<String>,
        service: Option<String>,
    ) -> Vec<String> {
        let mut filtered_sources = sources.clone();
        let mut filtered_iter_source = filtered_sources.iter();

        filtered_iter_source
            .filter(|s| {
                if category != None {
                    s.category == category
                } else {
                    true
                }
            })
            .filter(|s| {
                if product != None {
                    s.product == product
                } else {
                    true
                }
            })
            .filter(|s| {
                if service != None {
                    s.service == service
                } else {
                    true
                }
            })
            .map(|s| s.value.clone())
            .collect()
    }
}

pub struct SentinelQuery {
    query: String,
}

pub enum Condition {
    And,
    Or,
}

impl SentinelQuery {
    fn default() -> SentinelQuery {
        SentinelQuery {
            query: String::new(),
        }
    }

    pub fn new() -> SentinelQuery {
        SentinelQuery::default()
    }

    pub fn comment(&mut self, comment: &str) {
        self.query = format!("// {}\n{}", comment, self.query)
    }

    pub fn from(&mut self, source: &str) {
        self.query = String::from(source)
    }

    pub fn add_where(&mut self) {
        todo!()
    }

    pub fn extend(&mut self) {
        todo!()
    }

    pub fn join(first: &SentinelQuery, second: &SentinelQuery) -> SentinelQuery {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_init() {
        let query = SentinelQuery::new().query;
        assert_eq!(query, String::new());
    }

    #[test]
    fn test_comment() {
        let mut query = SentinelQuery::new();
        query.comment("This is a test comment");
        assert_eq!(query.query, "// This is a test comment\n".to_string());
    }

    #[test]
    fn test_source() {
        let mut query = SentinelQuery::new();
        query.from("CommonSecurity");
        query.comment("This is a test comment");
        assert_eq!(
            query.query,
            "// This is a test comment\nCommonSecurity".to_string()
        );
    }

    #[test]
    fn test_load_source() {
        let sources =
            SentinelLogSource::load_conf_file("./src/sentinel/tests/log_source.yml".to_string())
                .unwrap();
        assert_eq!(
            sources,
            vec![
                SentinelLogSource {
                    category: None,
                    service: None,
                    product: Some("windows".to_string()),
                    value: "windowsEvent".to_string(),
                },
                SentinelLogSource {
                    category: Some("firewall".to_string()),
                    service: None,
                    product: Some("zscaler".to_string()),
                    value: "CommonSecurity\n Where DeviceVendor == 'Zscaler'".to_string(),
                }
            ]
        );

        assert_eq!(
            SentinelLogSource::get_sources(&sources, None, Some("windows".to_string()), None),
            vec!["windowsEvent".to_string()]
        );
        assert_eq!(
            SentinelLogSource::get_sources(&sources, None, None, None),
            vec![
                "windowsEvent".to_string(),
                "CommonSecurity\n Where DeviceVendor == 'Zscaler'".to_string()
            ]
        );
        assert_eq!(
            SentinelLogSource::get_sources(&sources, Some("firewall".to_string()), None, None),
            vec!["CommonSecurity\n Where DeviceVendor == 'Zscaler'".to_string()]
        );
    }
}
