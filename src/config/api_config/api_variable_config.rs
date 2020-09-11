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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn create_api_variable_config_when_variable_array_is_only_one() {
        let config_yaml = yaml_rust::yaml::YamlLoader::load_from_str(r#"
        variable:
          - name: client1
            url: http://localhost/path/to/api
            body: "{\"test\":\"client1\"}"
        "#).unwrap();

        let result = ApiVariableConfig::load(&config_yaml[0]).unwrap();
        assert_eq!("client1".to_string(), result[0].name);
        assert_eq!("http://localhost/path/to/api".to_string(), result[0].url);
        assert_eq!("{\"test\":\"client1\"}".to_string(), result[0].body);
    }

    #[test]
    fn create_api_variable_config_when_variable_array_is_multiple() {
        let config_yaml = yaml_rust::yaml::YamlLoader::load_from_str(r#"
        variable:
          - name: client1
            url: http://localhost/path/to/api
            body: "{\"test\":\"client1\"}"
          - name: client2
            url: http://localhost/path/to/api2
            body: "{\"test\":\"client2\"}"
        "#).unwrap();

        let result = ApiVariableConfig::load(&config_yaml[0]).unwrap();
        assert_eq!("client1".to_string(), result[0].name);
        assert_eq!("http://localhost/path/to/api".to_string(), result[0].url);
        assert_eq!("{\"test\":\"client1\"}".to_string(), result[0].body);
        assert_eq!("client2".to_string(), result[1].name);
        assert_eq!("http://localhost/path/to/api2".to_string(), result[1].url);
        assert_eq!("{\"test\":\"client2\"}".to_string(), result[1].body);
    }

    #[test]
    fn create_api_variable_config_when_body_is_not_define() {
        let config_yaml = yaml_rust::yaml::YamlLoader::load_from_str(r#"
        variable:
          - name: client1
            url: http://localhost/path/to/api
        "#).unwrap();

        let result = ApiVariableConfig::load(&config_yaml[0]).unwrap();
        assert_eq!("client1".to_string(), result[0].name);
        assert_eq!("http://localhost/path/to/api".to_string(), result[0].url);
        assert_eq!("".to_string(), result[0].body);
    }
}
