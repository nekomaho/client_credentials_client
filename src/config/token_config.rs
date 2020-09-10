use crate::config::fetch_value::FetchValue;

#[derive(Debug, Clone)]
pub struct TokenConfig {
    pub token_url: String,
    pub scope: String,
}

impl FetchValue for TokenConfig {}

impl TokenConfig {
    pub fn new(config: &yaml_rust::yaml::Yaml) -> Result<Self, i32> {
        Ok(TokenConfig {
            token_url: TokenConfig::fetch_value(config, &vec!["token", "token_url"])?,
            scope: TokenConfig::fetch_value(config, &vec!["token", "scope"])?,
        })
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use linked_hash_map::LinkedHashMap;

    #[test]
    fn create_token_config() {
        let mut config_yaml_hash = LinkedHashMap::new();
        let token_element = yaml_rust::yaml::Yaml::String("token".to_string());
        let mut sub_config = LinkedHashMap::new();
        let token_url_element = yaml_rust::yaml::Yaml::String("token_url".to_string());
        let token_url_value = yaml_rust::yaml::Yaml::String("http://token_url".to_string());
        let scope_element = yaml_rust::yaml::Yaml::String("scope".to_string());
        let scope_value = yaml_rust::yaml::Yaml::String("test test2".to_string());

        sub_config.insert(token_url_element, token_url_value);
        sub_config.insert(scope_element, scope_value);
        let sub_config_hash = yaml_rust::yaml::Yaml::Hash(sub_config);

        config_yaml_hash.insert(token_element, sub_config_hash);
        let config_yaml = yaml_rust::yaml::Yaml::Hash(config_yaml_hash);

        let expected_value = TokenConfig {
            token_url: "http://token_url".to_string(),
            scope: "test test2".to_string()
        };

        let result = TokenConfig::new(&config_yaml).unwrap();
        assert_eq!(expected_value.token_url, result.token_url);
        assert_eq!(expected_value.scope, result.scope);
    }
}
