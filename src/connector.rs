use crate::{
    db::RedisRepo,
    dto::{GetUpdResp, TgUpdRespWrapper},
    types::{PollingCommand, UpdateData},
};
use anyhow::Result;
use reqwest::StatusCode;

pub struct TgConnector {
    pub api_url: String,
    pub token: String,
    pub client: reqwest::Client,
    pub repo: RedisRepo,
    pub polling_timeout: i64,
}

impl TgConnector {
    pub async fn get_update_msg(&self, offs_idx: i64) -> Result<PollingCommand> {
        let path: String = format!("{}/bot{}/getUpdates", self.api_url, self.token);

        let result = self
            .client
            .get(path)
            .query(&[("offset", offs_idx), ("timeout", self.polling_timeout)])
            .send()
            .await?;

        match result.status() {
            StatusCode::BAD_GATEWAY => Ok(PollingCommand::Continue(UpdateData::Timeout)),
            StatusCode::OK => {
                let data: TgUpdRespWrapper<GetUpdResp> = result.json().await?;

                Ok(PollingCommand::Continue(UpdateData::Data(Box::new(
                    data.result,
                ))))
            }
            _ => Ok(PollingCommand::Stop),
        }
    }
}
