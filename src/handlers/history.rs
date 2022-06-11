use std::sync::Arc;

use crate::{
    connector::TgConnector,
    dto::*,
    helpers::{parser::parse_msg_req, prop_gen::*},
    types::*,
};
use anyhow::Result;
use reqwest::Response;

pub struct HandleHistory {
    con: Arc<TgConnector>,
}

impl HandleHistory {
    pub fn new(con: Arc<TgConnector>) -> Self {
        Self { con }
    }

    pub async fn save_hist_id(&self, chat_id: i64, hist_msg_id: i64) -> Result<Option<i64>> {
        let old_msg: Option<i64> = self
            .con
            .repo
            .read("GET", vec![get_chat_last_hist_p_name(chat_id)])
            .await?;

        self.con
            .repo
            .write(
                "SET",
                vec![get_chat_last_hist_p_name(chat_id), hist_msg_id.to_string()],
            )
            .await?;

        Ok(old_msg)
    }

    async fn get_last_msg_id(&self, chat_id: i64) -> Result<Option<String>> {
        self.con
            .repo
            .read("GET", [get_chat_last_hist_msg_p_name(chat_id)].to_vec())
            .await
    }

    async fn get_last_msg_data(
        &self,
        last_msg_id: Option<String>,
        chat_id: i64,
    ) -> Result<Option<MsgObj>> {
        let read_msg_args: Vec<String>;

        match last_msg_id {
            Some(d) => {
                read_msg_args = vec![
                    get_chat_msgs_p_name(chat_id),
                    format!("{}", d),
                    String::from("-"),
                    String::from("COUNT"),
                    String::from("1"),
                ]
            }
            None => {
                read_msg_args = vec![
                    get_chat_msgs_p_name(chat_id),
                    String::from("+"),
                    String::from("-"),
                    String::from("COUNT"),
                    String::from("1"),
                ]
            }
        }

        let res: Option<redis::Value> = self.con.repo.read("XREVRANGE", read_msg_args).await?;
        let mut last_msg: Option<MsgObj> = None;

        if let Some(last_msg_bulk) = res {
            let mut parsed = parse_msg_req(last_msg_bulk)?;

            if !parsed.is_empty() {
                let first = parsed.swap_remove(0);
                self.con
                    .repo
                    .write(
                        "SET",
                        [get_chat_last_hist_msg_p_name(chat_id), first.id.to_string()].to_vec(),
                    )
                    .await?;

                last_msg = Some(first)
            }
        }

        Ok(last_msg)
    }

    pub async fn get_msg(&self, msg: Message) -> Result<HistMsgData> {
        let chat_id = msg.chat.id;

        let last_msg_id = self.get_last_msg_id(chat_id).await?;
        let last_msg = self.get_last_msg_data(last_msg_id, chat_id).await?;

        match last_msg {
            Some(m) => Ok(HistMsgData {
                payload: m.text,
                msg_id: Some(m.id),
                chat_id,
            }),
            None => Ok(HistMsgData {
                payload: "No messages yet".to_string(),
                msg_id: None,
                chat_id,
            }),
        }
    }

    pub async fn send_message(&self, msg: HistMsgData) -> Result<Response> {
        let path = format!(
            "{}/bot{}/{}",
            self.con.api_url,
            self.con.token,
            String::from("sendMessage")
        );
        let mut text: String = String::from("No messages yet");
        let mut kb_btns: Vec<InlineKeyboardButton> = Vec::with_capacity(2);

        if let Some(m_id) = msg.msg_id {
            text = msg.payload;
            kb_btns.push(InlineKeyboardButton {
                text: "Prev".to_string(),
                callback_data: format!("p::{}", &m_id),
            });
            kb_btns.push(InlineKeyboardButton {
                text: "Next".to_string(),
                callback_data: format!("n::{}", &m_id),
            });
        }

        let hist_msg = HistMsgWithInlKeyboard {
            text,
            chat_id: msg.chat_id,
            message_id: None,
            reply_markup: InlineKeyboard {
                inline_keyboard: [kb_btns],
            },
        };

        let ser_msg = serde_json::to_string(&hist_msg)?;

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

    pub async fn delete_message(&self, chat_id: i64, msg_id: i64) -> Result<Response> {
        let path = format!("{}/bot{}/deleteMessage", self.con.api_url, self.con.token);

        let res = self
            .con
            .client
            .get(path)
            .query(&[("chat_id", chat_id), ("message_id", msg_id)])
            .send()
            .await?;

        Ok(res)
    }
}
