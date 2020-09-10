use crate::config::fetch_value::FetchValue;

#[derive(Debug, Clone)]
pub struct ApiVariableConfig {
    pub name: String,
    pub url: String,
    pub body: String,
}

impl FetchValue for ApiVariableConfig {}

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
                &ApiVariableConfig::fetch_value_allow_empty(&variable_config, &vec!["body"]),
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
