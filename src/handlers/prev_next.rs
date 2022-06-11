use crate::{
    connector::TgConnector,
    dto::{HistMsgWithInlKeyboard, InlineKeyboard, InlineKeyboardButton},
    helpers::{parser::parse_msg_req, prop_gen::*},
    types::{KeyboardButtons, MsgObj},
};
use anyhow::{anyhow, Result};
use reqwest::Response;
use std::sync::Arc;

pub struct HandlePrevNext {
    con: Arc<TgConnector>,
}

impl HandlePrevNext {
    pub fn new(con: Arc<TgConnector>) -> Self {
        Self { con }
    }

    pub fn parse_cmd_msg_id<'a>(&self, payload: &'a str) -> Result<(KeyboardButtons, &'a str)> {
        let mut splitted = payload.split("::");

        let cmd = splitted
            .next()
            .ok_or(anyhow!("Inline data command parse error"))?;
        let cur_msg_id = splitted
            .next()
            .ok_or(anyhow!("Inline data id parse error"))?;

        let c = match cmd {
            "p" => KeyboardButtons::Prev,
            "n" => KeyboardButtons::Next,
            _ => KeyboardButtons::Unknown,
        };

        Ok((c, cur_msg_id))
    }

    pub async fn get_msg_data(
        &self,
        cmd: KeyboardButtons,
        msg_id: &str,
        chat_id: i64,
    ) -> Result<Option<MsgObj>> {
        let res_msg: Option<redis::Value>;

        match cmd {
            KeyboardButtons::Prev => {
                res_msg = self
                    .con
                    .repo
                    .read(
                        "XREVRANGE",
                        vec![
                            get_chat_msgs_p_name(chat_id),
                            format!("({}", msg_id),
                            String::from("-"),
                            String::from("count"),
                            String::from("1"),
                        ],
                    )
                    .await?;
            }
            KeyboardButtons::Next => {
                res_msg = self
                    .con
                    .repo
                    .read(
                        "XRANGE",
                        vec![
                            get_chat_msgs_p_name(chat_id),
                            format!("({}", msg_id),
                            String::from("+"),
                            String::from("count"),
                            String::from("1"),
                        ],
                    )
                    .await?;
            }
            KeyboardButtons::Unknown => return Err(anyhow!("Unknown query data prefix")),
        }

        let mut result = Ok(None);

        if let Some(msg) = res_msg {
            let mut parsed_msg = parse_msg_req(msg)?;

            if !parsed_msg.is_empty() {
                let parsed_msg = parsed_msg.swap_remove(0);
                result = Ok(Some(parsed_msg));
            }
        }

        result
    }

    pub async fn get_hist_msg_id(&self, chat_id: i64) -> Result<Option<String>> {
        self.con
            .repo
            .read("GET", vec![get_chat_last_hist_p_name(chat_id)])
            .await
    }

    pub async fn edit_message(
        &self,
        m: MsgObj,
        hist_msg_id: String,
        chat_id: i64,
    ) -> Result<Response> {
        let path = format!("{}/bot{}/editMessageText", self.con.api_url, self.con.token);

        let new_msg = HistMsgWithInlKeyboard {
            text: m.text,
            chat_id,
            message_id: Some(hist_msg_id),
            reply_markup: InlineKeyboard {
                inline_keyboard: [vec![
                    InlineKeyboardButton {
                        text: "Prev".to_string(),
                        callback_data: format!("p::{}", &m.id),
                    },
                    InlineKeyboardButton {
                        text: "Next".to_string(),
                        callback_data: format!("n::{}", &m.id),
                    },
                ]],
            },
        };

        let ser_msg = serde_json::to_string(&new_msg)?;

        let res = self
            .con
            .client
            .post(path)
            .header("Content-Type", "application/json")
            .body(ser_msg)
            .send()
            .await?;

        Ok(res)
    }
}
