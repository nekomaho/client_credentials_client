mod fetch_value;
pub mod oauth_config;
pub mod token_config;
pub mod api_config;

use std::fs;
use yaml_rust::YamlLoader;
use crate::config::oauth_config::OauthConfig;
use crate::config::token_config::TokenConfig;
use crate::config::api_config::ApiConfig;

#[derive(Debug, Clone)]
pub struct Config {
    pub oauth: Vec<OauthConfig>,
    pub token: TokenConfig,
    pub api: Vec<ApiConfig>,
}

const CONFIG_FILE: &str = "client_credentials_client.yml";

impl Config {
    pub fn new() -> Result<Self, i32> {
        let config_content = match fs::read_to_string(CONFIG_FILE) {
            Ok(result) => result,
            Err(_) => {
                println!("Need {} at current directory", CONFIG_FILE);
                return Err(1);
            }
        };

        let config_yaml = match YamlLoader::load_from_str(&config_content) {
            Ok(result) => result,
            Err(_) => {
                println!("Cannot parse {}", CONFIG_FILE);
                return Err(1);
            }
        };

        let config = &config_yaml[0];

        Ok(Config {
            oauth: OauthConfig::load(config)?,
            token: TokenConfig::new(config)?,
            api: ApiConfig::load(config)?,
        })
    }

}
