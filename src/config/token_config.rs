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
