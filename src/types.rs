use crate::dto::*;

pub enum UpdateData {
    Timeout,
    Data(Box<Vec<GetUpdResp>>),
}

pub enum KeyboardButtons {
    Unknown,
    Prev,
    Next,
}

pub enum PollingCommand {
    Continue(UpdateData),
    Stop,
}

pub enum MsgType {
    Msg(Message),
    History(Message),
    PrevNext((String, i64)),
    Unknown,
}

#[derive(Debug)]
pub struct MsgObj {
    pub id: String,
    pub text: String,
}

pub struct HistMsgData {
    pub payload: String,
    pub msg_id: Option<String>,
    pub chat_id: i64,
}
