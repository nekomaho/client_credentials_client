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

    #[test]
    fn create_token_config() {
        let config_yaml = yaml_rust::yaml::YamlLoader::load_from_str("
        token:
          token_url: http://token_url
          scope: test test2
        ").unwrap();

        let expected_value = TokenConfig {
            token_url: "http://token_url".to_string(),
            scope: "test test2".to_string()
        };

        let result = TokenConfig::new(&config_yaml[0]).unwrap();
        assert_eq!(expected_value.token_url, result.token_url);
        assert_eq!(expected_value.scope, result.scope);
    }
}
