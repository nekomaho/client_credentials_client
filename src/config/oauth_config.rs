use std::env;
use crate::config::fetch_value::FetchValue;

#[derive(Debug, Clone)]
pub struct OauthConfig {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub count: u32,
}

impl FetchValue for OauthConfig {}

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
