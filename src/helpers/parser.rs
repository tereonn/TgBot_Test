use crate::{
    dto::GetUpdResp,
    types::{MsgObj, MsgType},
};
use anyhow::{anyhow, Result};

pub fn parse_resp(resp: GetUpdResp) -> MsgType {
    if resp.callback_query.is_some() {
        let cb_query = resp.callback_query.unwrap();

        return MsgType::PrevNext((cb_query.data, cb_query.message.chat.id));
    }

    if resp.message.is_some() {
        let msg = resp.message.unwrap();
        if let Some(ref text) = msg.text {
            match text.as_str() {
                "/history" => return MsgType::History(msg),
                _ => return MsgType::Msg(msg),
            }
        }
    }

    MsgType::Unknown
}

pub fn parse_msg_req(v: redis::Value) -> Result<Vec<MsgObj>> {
    let mut result: Vec<MsgObj> = Vec::new();

    for element in v.as_sequence().ok_or(anyhow!("Parse err"))? {
        for (id_part, msg_part) in element.as_map_iter().ok_or(anyhow!("Parse err"))? {
            if let redis::Value::Data(raw_id) = id_part {
                for (_, text_part) in msg_part.as_map_iter().ok_or(anyhow!("Parse err"))? {
                    if let redis::Value::Data(raw_text) = text_part {
                        result.push(MsgObj {
                            id: std::str::from_utf8(raw_id)?.to_owned().parse()?,
                            text: std::str::from_utf8(raw_text)?.to_owned(),
                        });
                    }
                }
            }
        }
    }

    Ok(result)
}
