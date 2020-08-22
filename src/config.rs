use std::fs;
use yaml_rust::YamlLoader;

#[derive(Debug, Clone)]
pub struct Config {
    pub oauth: OauthConfig,
    pub token: TokenConfig,
    pub api: ApiConfig,
}

trait FetchValue {
    fn fetch_value(
        config_yaml: &yaml_rust::yaml::Yaml,
        config_type: &str,
        element: &str,
    ) -> Result<String, i32> {
        match config_yaml[config_type][element].as_str() {
            Some(value) => Ok(value.to_string()),
            None => {
                println!("{} is empty", element);
                Err(1)
            }
        }
    }

    fn fetch_value_allow_empty(
        config_yaml: &yaml_rust::yaml::Yaml,
        config_type: &str,
        element: &str,
    ) -> String {
        match config_yaml[config_type][element].as_str() {
            Some(value) => value.to_string(),
            None => "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OauthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub token_url: String,
}

impl FetchValue for OauthConfig {}

#[derive(Debug, Clone)]
pub struct TokenConfig {
    pub scope: String,
}

impl FetchValue for TokenConfig {}

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub method: String,
    pub url: String,
    pub content_type: String,
    pub body: String,
}

impl FetchValue for ApiConfig {}

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
            oauth: OauthConfig::new(config)?,
            token: TokenConfig::new(config)?,
            api: ApiConfig::new(config)?,
        })
    }
}

impl OauthConfig {
    pub fn new(config: &yaml_rust::yaml::Yaml) -> Result<Self, i32> {
        Ok(OauthConfig {
            client_id: OauthConfig::fetch_value(config, "oauth", "client_id")?,
            client_secret: OauthConfig::fetch_value(config, "oauth", "client_secret")?,
            token_url: OauthConfig::fetch_value(config, "oauth", "token_url")?,
        })
    }
}

impl TokenConfig {
    pub fn new(config: &yaml_rust::yaml::Yaml) -> Result<Self, i32> {
        Ok(TokenConfig {
            scope: TokenConfig::fetch_value(config, "token", "scope")?,
        })
    }
}

impl ApiConfig {
    pub fn new(config: &yaml_rust::yaml::Yaml) -> Result<Self, i32> {
        Ok(ApiConfig {
            method: ApiConfig::fetch_value(config, "api", "method")?,
            url: ApiConfig::fetch_value(config, "api", "url")?,
            content_type: ApiConfig::fetch_value(config, "api", "content_type")?,
            body: ApiConfig::fetch_value_allow_empty(config, "api", "body"),
        })
    }
}
