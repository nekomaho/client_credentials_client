use reqwest::header::*;

pub struct Api {
    config: crate::config::Config,
    oauth_name: String,
    access_token: String,
}

impl Api {
    pub fn new(config: &crate::config::Config, access_token: &str, oauth_name: &str) -> Self {
        Api {
            config: config.clone(),
            oauth_name: oauth_name.to_string(),
            access_token: access_token.to_string(),
        }
    }

    pub fn send_request(&self) -> Result<i32, i32> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, self.config.api.content_type.parse().unwrap());

        Ok(match &*self.config.api.method {
            "get" => self.get(headers)?,
            "post" => self.post(headers)?,
            _ => {
                println!("{} is not support method", self.config.api.method);
                return Err(1);
            }
        })
    }

    #[tokio::main]
    async fn post(&self, headers: HeaderMap) -> Result<i32, i32> {
        let req = reqwest::Client::new();
        let body = self.config.api.body.get(&self.oauth_name).unwrap();
        let res = req
            .post(&self.config.api.url)
            .bearer_auth(self.access_token.to_string())
            .headers(headers)
            .body(body.to_string())
            .send()
            .await;
        let res_result = match res {
            Ok(result) => result,
            Err(err) => {
                let status_code = match err.status() {
                    Some(code) => code.to_string(),
                    None => "".to_string(),
                };
                println!(
                    "post error {}: response={}",
                    &self.config.api.url, status_code
                );
                return Err(1);
            }
        };

        println!("{}", res_result.status());
        println!("{}", res_result.text().await.unwrap());

        Ok(0)
    }

    #[tokio::main]
    async fn get(&self, headers: HeaderMap) -> Result<i32, i32> {
        let req = reqwest::Client::new();
        let res = req
            .get(&self.config.api.url)
            .bearer_auth(self.access_token.to_string())
            .headers(headers)
            .send()
            .await;
        let res_result = match res {
            Ok(result) => result,
            Err(err) => {
                let status_code = match err.status() {
                    Some(code) => code.to_string(),
                    None => "".to_string(),
                };
                println!(
                    "get error {}: response={}",
                    &self.config.api.url, status_code
                );
                return Err(1);
            }
        };

        println!("{}", res_result.status());
        println!("{}", res_result.text().await.unwrap());

        Ok(0)
    }
}
