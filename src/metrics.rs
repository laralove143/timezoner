use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sparkle_convenience::error::IntoError;

use crate::{Context, Error};

const APPLICATION_INFO_URL: &str = "https://discord.com/api/v10/applications/@me";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metrics {
    guild_count: u64,
}

#[derive(Debug)]
pub struct JsonStorageClient {
    pub http: reqwest::Client,
    pub api_key: String,
    pub url: String,
}

impl Context {
    pub async fn update_metrics(&self) -> Result<()> {
        let guild_count = self
            .json_storage
            .http
            .get(APPLICATION_INFO_URL)
            .header("Authorization", self.bot.http.token().ok()?)
            .send()
            .await?
            .json::<serde_json::Map<String, Value>>()
            .await?
            .get("approximate_guild_count")
            .ok()?
            .as_u64()
            .ok()?;

        let put_metrics = Metrics { guild_count };

        self.json_storage
            .http
            .put(&self.json_storage.url)
            .query(&[("apiKey", &self.json_storage.api_key)])
            .json(&put_metrics)
            .send()
            .await?;

        let get_metrics = self
            .json_storage
            .http
            .get(&self.json_storage.url)
            .send()
            .await?
            .json::<Metrics>()
            .await?;

        if get_metrics != put_metrics {
            return Err(Error::MetricsUpdateFail {
                get: get_metrics,
                put: put_metrics,
            }
            .into());
        }

        Ok(())
    }
}
