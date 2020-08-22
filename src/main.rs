mod config;
mod api;

use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{AuthUrl, ClientId, ClientSecret, Scope, TokenResponse, TokenUrl};

fn run() -> Result<i32, i32> {
    let config = config::Config::new()?;

    let client_secret = &config.oauth.client_secret;
    let client_id = &config.oauth.client_id;
    let token_url = &config.oauth.token_url;
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        AuthUrl::new("http://dummy".to_string()).unwrap(),
        Some(TokenUrl::new(token_url.to_string()).unwrap()),
    );

    let scope = &config.token.scope;
    let token_result = client
        .exchange_client_credentials()
        .add_scope(Scope::new(scope.to_string()))
        .request(http_client);

    let secret = match token_result {
        Ok(result) => result.access_token().secret().to_string(),
        Err(err) => {
            println!("get access token error: {}", err);
            return Err(1)
        }
    };

    let api = api::Api::new(&config, &secret);
    api.send_request()?;
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
