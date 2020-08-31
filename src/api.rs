use reqwest::header::*;
use crate::color::coloring;

pub struct Api {
    config: crate::config::api_config::ApiConfig,
    oauth_name: String,
    access_token: String,
    count: u32,
}

impl Api {
    pub fn new(config: &crate::config::api_config::ApiConfig, access_token: &str, oauth_name: &str, count: u32) -> Self {
        Api {
            config: config.clone(),
            oauth_name: oauth_name.to_string(),
            access_token: access_token.to_string(),
            count: count,
        }
    }

    pub fn send_request(&self) -> Result<i32, i32> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, self.config.content_type.parse().unwrap());

        Ok(match &*self.config.method {
            "get" => self.get(headers)?,
            "post" => self.post(headers)?,
            _ => {
                println!("{} is not support method", self.config.method);
                return Err(1);
            }
        })
    }

    #[tokio::main]
    async fn post(&self, headers: HeaderMap) -> Result<i32, i32> {
        let req = reqwest::Client::new();
        let body = &self.config.variable.get(&self.oauth_name).unwrap().body;
        let res = req
            .post(&self.config.variable.get(&self.oauth_name).unwrap().url)
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
                    &self.config.variable.get(&self.oauth_name).unwrap().url, status_code
                );
                return Err(1);
            }
        };

        let recv_status_output = format!("RECV: {} status={}", &self.oauth_name, &res_result.status());
        println!("{}", coloring(&recv_status_output, self.count));
        let recv_body_output = format!("RECV: {} body={}", &self.oauth_name, &res_result.text().await.unwrap());
        println!("{}", coloring(&recv_body_output, self.count));

        Ok(0)
    }

    #[tokio::main]
    async fn get(&self, headers: HeaderMap) -> Result<i32, i32> {
        let req = reqwest::Client::new();
        let res = req
            .get(&self.config.variable.get(&self.oauth_name).unwrap().url)
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
                    &self.config.variable.get(&self.oauth_name).unwrap().url, status_code
                );
                return Err(1);
            }
        };

        let recv_status_output = format!("RECV: {} status={}", &self.oauth_name, &res_result.status());
        println!("{}", coloring(&recv_status_output, self.count));
        let recv_body_output = format!("RECV: {} body={}", &self.oauth_name, &res_result.text().await.unwrap());
        println!("{}", coloring(&recv_body_output, self.count));

        Ok(0)
    }
}
