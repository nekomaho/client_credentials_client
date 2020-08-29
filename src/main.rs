mod api;
mod color;
mod config;

use color::coloring;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{AuthUrl, ClientId, ClientSecret, Scope, TokenResponse, TokenUrl};
use std::sync::Arc;
use std::thread;

fn run() -> Result<i32, i32> {
    let search_config = Arc::new(config::Config::new()?);
    let mut secrets = Vec::new();
    let mut results: Vec<std::thread::JoinHandle<Result<i32, i32>>> = Vec::new();

    for oauth_config_setting in &search_config.oauth {
        let count = oauth_config_setting.count;
        let client_secret = &oauth_config_setting.client_secret;
        let client_id = &oauth_config_setting.client_id;
        let token_url = &oauth_config_setting.token_url;

        let start_preparation_output = format!("START GET TOKEN: {}", &oauth_config_setting.name);
        println!("{}", coloring(&start_preparation_output, count));

        let client = BasicClient::new(
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
            AuthUrl::new("http://dummy".to_string()).unwrap(),
            Some(TokenUrl::new(token_url.to_string()).unwrap()),
        );

        let scope = &search_config.token.scope;
        let token_result = client
            .exchange_client_credentials()
            .add_scope(Scope::new(scope.to_string()))
            .request(http_client);

        let secret = match token_result {
            Ok(result) => result.access_token().secret().to_string(),
            Err(err) => {
                println!("get access token error: {}", err);
                return Err(1);
            }
        };

        let end_preparation_output = format!("END GET TOKEN: {}", &oauth_config_setting.name);
        println!("{}", coloring(&end_preparation_output, count));

        secrets.push(secret);
    }

    println!("START PARALLEL");

    for set_oauth_config_setting in &search_config.oauth {
        let config = search_config.clone();
        let oauth_config_setting = Arc::new(set_oauth_config_setting.clone());
        let count = oauth_config_setting.count;
        let secret = Arc::new(secrets[count as usize].clone());

        let handle = thread::spawn(move || {
            let start_output = format!("START: {} phase", &oauth_config_setting.name);
            println!("{}", coloring(&start_output, count));

            for api_config in &config.api {
                let send_output = format!(
                    "SEND: {} {}",
                    &oauth_config_setting.name, &api_config.api_name
                );
                println!("{}", coloring(&send_output, count));
                let api = api::Api::new(&api_config, &secret, &oauth_config_setting.name, count);
                api.send_request()?;
            }

            let end_output = format!("END: {} phase", &oauth_config_setting.name);
            println!("{}", coloring(&end_output, count));

            Ok(0)
        });
        results.push(handle);
    }

    for result in results {
        match result.join() {
            Ok(_) => (),
            Err(_) => {
                println!("thread failed");
                return Err(1);
            }
        }
    }

    println!("END PARALLEL");

    Ok(0)
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(code) => {
            std::process::exit(code);
        }
    };
}
