use crate::color::coloring;
use crate::color_println;
use crate::config;
use crate::api;

use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{AuthUrl, ClientId, ClientSecret, Scope, TokenResponse, TokenUrl};
use std::sync::Arc;
use std::thread;

pub fn request(config_file_name: &str) -> Result<i32, i32> {
    let search_config = Arc::new(config::Config::new(config_file_name)?);
    let mut secrets: Vec<String> = Vec::new();

    get_token(&search_config, &mut secrets)?;

    println!("START PARALLEL");

    send_request_parallel(&search_config, &secrets)?;

    println!("END PARALLEL");

    Ok(0)
}

fn get_token(search_config: &Arc<config::Config>,secrets: &mut Vec<String>) -> Result<i32, i32>{
    for oauth_config_setting in &search_config.oauth {
        let count = oauth_config_setting.count;
        let client_secret = &oauth_config_setting.client_secret;
        let client_id = &oauth_config_setting.client_id;
        let token_url = &search_config.token.token_url;

        color_println!(count, "START GET TOKEN: {}", &oauth_config_setting.name);

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
                match err {
                    oauth2::RequestTokenError::Request(_e) => {
                        println!("response error: client does not exist or network error");
                    },
                    oauth2::RequestTokenError::ServerResponse(e) => {
                        println!("server return error");
                        println!("---");
                        println!("{}", e.error());
                        println!("{}", e.error_description().unwrap());
                        println!("---");
                    },
                    _ => println!("some error"),
                }
                return Err(1);
            }
        };

        color_println!(count, "END GET TOKEN: {}", &oauth_config_setting.name);

        secrets.push(secret);
    }
    Ok(0)
}

fn send_request_parallel(search_config: &Arc<config::Config>,secrets: &Vec<String>) -> Result<i32, i32> {
    let mut results: Vec<std::thread::JoinHandle<Result<i32, i32>>> = Vec::new();

    for set_oauth_config_setting in &search_config.oauth {
        let config = search_config.clone();
        let oauth_config_setting = Arc::new(set_oauth_config_setting.clone());
        let count = oauth_config_setting.count;
        let secret = Arc::new(secrets[count as usize].clone());

        let handle = thread::spawn(move || {
            color_println!(count, "START: {} phase", &oauth_config_setting.name);

            for api_config in &config.api {
                if api_config.enable {
                    color_println!(count, "SEND: {} {}", &oauth_config_setting.name, &api_config.api_name);
                    let api = api::Api::new(&api_config, &secret, &oauth_config_setting.name, count);
                    api.send_request()?;
                } else {
                    color_println!(count, "SKIP: {} {}", &oauth_config_setting.name, &api_config.api_name);
                }
            }

            color_println!(count, "END: {} phase", &oauth_config_setting.name);

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
    Ok(0)
}
