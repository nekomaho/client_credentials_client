pub trait FetchValue {
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

    fn fetch_value_allow_empty(
        config_yaml: &yaml_rust::yaml::Yaml,
        config_types: &Vec<&str>,
    ) -> String {
        let mut config = config_yaml;
        for config_type in config_types {
            config = &config[*config_type];
        }

        match config.as_str() {
            Some(value) => value.to_string(),
            None => "".to_string()
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
}
