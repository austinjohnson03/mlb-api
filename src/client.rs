use crate::config::Config;
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::time::{Duration, sleep};

#[derive(Clone)]
pub struct ApiClient {
    http: Client,
    config: Config,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            http: Client::new(),
            config: Config::new(),
        }
    }

    pub async fn get<T: DeserializeOwned, P: Serialize>(
        &self,
        path: &str,
        params: Option<&P>,
    ) -> Result<T, String> {
        if let Some(ms) = self.config.rate_limit_ms {
            sleep(Duration::from_millis(ms)).await;
        }

        let url = format!("{}/{}", self.config.host, path);

        let to_err = |e: reqwest::Error| e.to_string();

        let resp = match params {
            Some(p) => self.http.get(&url).query(&p).send().await.map_err(to_err)?,
            None => self.http.get(&url).send().await.map_err(to_err)?,
        };

        match resp.status().as_u16() {
            200..=299 => {
                let body = resp.text().await.map_err(to_err)?;
                serde_json::from_str::<T>(&body).map_err(|e| format!("Error: {}", e))
            }
            status => Err(format!("HTTP Error: {}", status)),
        }
    }
}
