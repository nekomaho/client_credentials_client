use std::fs;
use yaml_rust::YamlLoader;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Config {
    pub oauth: Vec<OauthConfig>,
    pub token: TokenConfig,
    pub api: ApiConfig,
}

trait FetchValue {
    fn fetch_value(
        config_yaml: &yaml_rust::yaml::Yaml,
        config_types: &Vec<&str>
    ) -> Result<String, i32> {
        let mut config = config_yaml;
        for config_type in config_types {
          config =  &config[*config_type];
        }

        match config.as_str() {
            Some(value) => Ok(value.to_string()),
            None => {
                let mut output = String::new(); 
                for config_type in config_types {
                    output = format!("{} {}", config_type, output);
                }
                println!("{} is empty", output);
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
    pub name: String,
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
    pub body: HashMap<String, String>,
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
            oauth: OauthConfig::load(config)?,
            token: TokenConfig::new(config)?,
            api: ApiConfig::new(config)?,
        })
    }
}

impl OauthConfig {
    pub fn load(config: &yaml_rust::yaml::Yaml) -> Result<Vec<Self>, i32> {
        let mut oauth_settings: Vec<Self> = Vec::new();
        let oauth_vec = match config["oauth"].as_vec() {
            Some(result) => result,
            None => {
                println!("Not found oauth hash");
                return Err(1)
            }
        };

        for config_element in oauth_vec {
            let oauth = OauthConfig::new(
                &OauthConfig::fetch_value(&config_element, &vec!["name"])?,
                &OauthConfig::fetch_value(&config_element, &vec!["client_id"])?,
                &OauthConfig::fetch_value(&config_element, &vec!["client_secret"])?,
                &OauthConfig::fetch_value(&config_element, &vec!["token_url"])?,
            );
            oauth_settings.push(oauth);
        }

        Ok(oauth_settings)
    }

    pub fn new(name: &str, client_id: &str, client_secret: &str, token_url: &str) -> Self {
        OauthConfig {
            name: name.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            token_url: token_url.to_string(),
        }
    }
}

impl TokenConfig {
    pub fn new(config: &yaml_rust::yaml::Yaml) -> Result<Self, i32> {
        Ok(TokenConfig {
            scope: TokenConfig::fetch_value(config, &vec!["token", "scope"])?,
        })
    }
}

impl ApiConfig {
    pub fn new(config: &yaml_rust::yaml::Yaml) -> Result<Self, i32> {
        let mut bodies = HashMap::new();
        let empty_body = &Vec::new();
        let api_body_config = match config["api"]["body"].as_vec() {
            Some(result) => result,
            None => empty_body
        };
        for request in api_body_config {
            bodies.insert(
                ApiConfig::fetch_value(request, &vec!["name"])?,
                match request["request"].as_str() {
                    Some(result) => result.to_string(),
                    None => "".to_string(),
                }
            );
        }

        Ok(ApiConfig {
            method: ApiConfig::fetch_value(config, &vec!["api", "method"])?,
            url: ApiConfig::fetch_value(config, &vec!["api", "url"])?,
            content_type: ApiConfig::fetch_value(config, &vec!["api", "content_type"])?,
            body: bodies,
        })
    }
}
