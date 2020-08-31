pub mod api_variable_config;

use std::collections::HashMap;
use crate::config::api_config::api_variable_config::ApiVariableConfig;
use crate::config::fetch_value::FetchValue;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub api_name: String,
    pub method: String,
    pub content_type: String,
    pub variable: HashMap<String, ApiVariableConfig>,
}

impl FetchValue for ApiConfig {}

impl ApiConfig {
    pub fn load(config: &yaml_rust::yaml::Yaml) -> Result<Vec<Self>, i32> {
        let mut api_settings: Vec<Self> = Vec::new();
        let api_vec = match config["api"].as_vec() {
            Some(result) => result,
            None => {
                println!("Not found api array");
                return Err(1);
            }
        };

        for config_element in api_vec {
            let api = ApiConfig::new(
                &ApiConfig::fetch_value(&config_element, &vec!["api_name"])?,
                &ApiConfig::fetch_value(&config_element, &vec!["method"])?,
                &ApiConfig::fetch_value(&config_element, &vec!["content_type"])?,
                config_element,
            );
            let api_config = match api {
                Ok(result) => result,
                Err(_) => {
                    println!("Not found api element");
                    return Err(1);
                }
            };
            api_settings.push(api_config);
        }

        Ok(api_settings)
    }

    pub fn new(
        api_name: &str,
        method: &str,
        content_type: &str,
        config: &yaml_rust::yaml::Yaml,
    ) -> Result<Self, i32> {
        let api_variable_configs = match ApiVariableConfig::load(config) {
            Ok(result) => result,
            Err(_) => {
                println!("Not found api element");
                return Err(1);
            }
        };

        let mut api_variable_config_hash = HashMap::new();
        for api_variable in api_variable_configs {
            api_variable_config_hash.insert(api_variable.name.to_string(), api_variable);
        }

        Ok(ApiConfig {
            api_name: api_name.to_string(),
            method: method.to_string(),
            content_type: content_type.to_string(),
            variable: api_variable_config_hash,
        })
    }
}
