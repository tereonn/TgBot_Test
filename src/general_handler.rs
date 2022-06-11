use crate::dto::*;
use crate::handlers::common::HandleCommonMsgRedis;
use crate::handlers::history::HandleHistory;
use crate::handlers::prev_next::HandlePrevNext;
use crate::types::*;
use crate::TgConnector;
use anyhow::{anyhow, Result};
use reqwest::StatusCode;
use std::sync::Arc;

pub fn handle_msg(con: &Arc<TgConnector>, mt: MsgType) -> tokio::task::JoinHandle<Result<()>> {
    let con = con.clone();

    tokio::spawn(async move {
        match mt {
            MsgType::Msg(m) => {
                HandleCommonMsgRedis::new(con.repo.clone())
                    .save_msg(m)
                    .await?;
            }
            MsgType::History(m) => {
                let id = m.chat.id;
                let hdlr = HandleHistory::new(con);
                let last_msg = hdlr.get_msg(m).await?;
                let send_res = hdlr.send_message(last_msg).await?;

                match send_res.status() {
                    StatusCode::OK => {
                        let data: TgSendMsgRespWrapper = send_res.json().await?;

                        let new_hist_msg_id = data.result.message_id;

                        let old_hist_msg_id = hdlr.save_hist_id(id, new_hist_msg_id).await?;
                        if let Some(old_hist_msg_id) = old_hist_msg_id {
                            hdlr.delete_message(id, old_hist_msg_id).await?;
                        }

                        Ok(())
                    }
                    _ => Err(anyhow!("Hist msg show error")),
                }?;
            }
            MsgType::PrevNext((id, chat_id)) => {
                let hdlr = HandlePrevNext::new(con);
                let (cmd, msg_id) = hdlr.parse_cmd_msg_id(&id)?;
                let targ_msg = hdlr.get_msg_data(cmd, msg_id, chat_id).await?;

                if let Some(targ_msg) = targ_msg {
                    let hist_msg_id = hdlr.get_hist_msg_id(chat_id).await?;

                    if let Some(hist_msg_id) = hist_msg_id {
                        let upd_res = hdlr.edit_message(targ_msg, hist_msg_id, chat_id).await?;

                        if upd_res.status() != StatusCode::OK {
                            return Err(anyhow!("Update message error"));
                        }
                    } else {
                        return Err(anyhow!("Trying to press button without hist msg"));
                    }
                }
            }
            MsgType::Unknown => (),
        }

        Ok(())
    })
}
