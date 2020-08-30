use std::collections::HashMap;
use std::fs;
use std::env;
use yaml_rust::YamlLoader;

#[derive(Debug, Clone)]
pub struct Config {
    pub oauth: Vec<OauthConfig>,
    pub token: TokenConfig,
    pub api: Vec<ApiConfig>,
}

trait FetchValue {
    fn fetch_value(
        config_yaml: &yaml_rust::yaml::Yaml,
        config_types: &Vec<&str>,
    ) -> Result<String, i32> {
        let mut config = config_yaml;
        for config_type in config_types {
            config = &config[*config_type];
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

    fn fetch_value_as_bool(
        config_yaml: &yaml_rust::yaml::Yaml,
        config_types: &Vec<&str>,
    ) -> Result<bool, i32> {
        let mut config = config_yaml;
        for config_type in config_types {
            config = &config[*config_type];
        }

        match config.as_bool() {
            Some(value) => Ok(value),
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
    pub count: u32,
}

impl FetchValue for OauthConfig {}

#[derive(Debug, Clone)]
pub struct TokenConfig {
    pub token_url: String,
    pub scope: String,
}

impl FetchValue for TokenConfig {}

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub api_name: String,
    pub method: String,
    pub content_type: String,
    pub variable: HashMap<String, ApiVariableConfig>,
}

impl FetchValue for ApiConfig {}

#[derive(Debug, Clone)]
pub struct ApiVariableConfig {
    pub name: String,
    pub url: String,
    pub body: String,
}

impl FetchValue for ApiVariableConfig {}

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

impl OauthConfig {
    pub fn load(config: &yaml_rust::yaml::Yaml) -> Result<Vec<Self>, i32> {
        let mut oauth_settings: Vec<Self> = Vec::new();
        let oauth_vec = match config["oauth"].as_vec() {
            Some(result) => result,
            None => {
                println!("Not found oauth hash");
                return Err(1);
            }
        };
        let mut counter = 0;

        for config_element in oauth_vec {
            let env = &OauthConfig::fetch_value_as_bool(&config_element, &vec!["env"])?;

            let name = &OauthConfig::fetch_value(&config_element, &vec!["name"])?;
            let client_id = match *env {
                true => {
                    let env_value = &OauthConfig::fetch_value(&config_element, &vec!["client_id"])?;
                    match env::var(env_value) {
                        Ok(result) => result.to_string(),
                        Err(_) => {
                            println!("{} is not found", env_value);
                            return Err(1)
                        }

                    }
                },
                false => OauthConfig::fetch_value(&config_element, &vec!["client_id"])?.to_string()
            };

            let client_secret = match *env {
                true => {
                    let env_value = &OauthConfig::fetch_value(&config_element, &vec!["client_secret"])?;
                    match env::var(env_value) {
                        Ok(result) => result.to_string(),
                        Err(_) => {
                            println!("{} is not found", env_value);
                            return Err(1)
                        }
                    }
                },
                false => OauthConfig::fetch_value(&config_element, &vec!["client_secret"])?.to_string()
            };

            let oauth = OauthConfig::new(
                name,
                &client_id,
                &client_secret,
                counter
            );
            oauth_settings.push(oauth);
            counter += 1;
        }

        Ok(oauth_settings)
    }

    pub fn new(name: &str, client_id: &str, client_secret: &str, counter: u32) -> Self {
        OauthConfig {
            name: name.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            count: counter
        }
    }
}

impl TokenConfig {
    pub fn new(config: &yaml_rust::yaml::Yaml) -> Result<Self, i32> {
        Ok(TokenConfig {
            token_url: TokenConfig::fetch_value(config, &vec!["token", "token_url"])?,
            scope: TokenConfig::fetch_value(config, &vec!["token", "scope"])?,
        })
    }
}

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

impl ApiVariableConfig {
    pub fn load(config: &yaml_rust::yaml::Yaml) -> Result<Vec<Self>, i32> {
        let mut api_variable_configs: Vec<Self> = Vec::new();
        let api_variables = match config["variable"].as_vec() {
            Some(result) => result,
            None => {
                println!("Need variable field");
                return Err(1);
            }
        };

        for variable_config in api_variables {
            let api_variable_config = ApiVariableConfig::new(
                &ApiVariableConfig::fetch_value(&variable_config, &vec!["name"])?,
                &ApiVariableConfig::fetch_value(&variable_config, &vec!["url"])?,
                &ApiVariableConfig::fetch_value(&variable_config, &vec!["body"])?,
            );
            api_variable_configs.push(api_variable_config);
        }

        Ok(api_variable_configs)
    }

    pub fn new(name: &str, url: &str, body: &str) -> Self {
        ApiVariableConfig {
            name: name.to_string(),
            url: url.to_string(),
            body: body.to_string(),
        }
    }
}
