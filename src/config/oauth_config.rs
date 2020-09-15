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

#[cfg(test)]
mod test{
    use super::*;
    use std::env::set_var;

    #[test]
    fn load_oauth_values_when_env_is_false() {
        let config_yaml = yaml_rust::yaml::YamlLoader::load_from_str("
        oauth:
          - name: test1
            env: false
            client_id: id 
            client_secret: secret 
          - name: test2
            env: false
            client_id: id2
            client_secret: secret2
        ").unwrap();

        let result = OauthConfig::load(&config_yaml[0]).unwrap();
        assert_eq!("test1", result[0].name);
        assert_eq!("id", result[0].client_id);
        assert_eq!("secret", result[0].client_secret);
        assert_eq!("test2", result[1].name);
        assert_eq!("id2", result[1].client_id);
        assert_eq!("secret2", result[1].client_secret);
    }

    #[test]
    fn load_oauth_values_when_env_is_true() {

        let config_yaml = yaml_rust::yaml::YamlLoader::load_from_str("
        oauth:
          - name: test1
            env: true
            client_id: TEST_ID1
            client_secret: TEST_SECRET1
          - name: test2
            env: true
            client_id: TEST_ID2
            client_secret: TEST_SECRET2
        ").unwrap();

        set_var("TEST_ID1","id");
        set_var("TEST_SECRET1","secret");
        set_var("TEST_ID2","id2");
        set_var("TEST_SECRET2","secret2");


        let result = OauthConfig::load(&config_yaml[0]).unwrap();
        assert_eq!("test1", result[0].name);
        assert_eq!("id", result[0].client_id);
        assert_eq!("secret", result[0].client_secret);
        assert_eq!("test2", result[1].name);
        assert_eq!("id2", result[1].client_id);
        assert_eq!("secret2", result[1].client_secret);
    }
}
