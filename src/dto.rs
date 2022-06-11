use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetMeResp {
    pub is_bot: bool,
    pub id: i64,
    pub first_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TgRespWrapper<T> {
    pub ok: bool,
    pub result: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TgUpdRespWrapper<T> {
    pub ok: bool,
    pub result: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TgSendMsgRespWrapper {
    pub ok: bool,
    pub result: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub text: Option<String>,
    pub chat: Chat,
    pub reply_markup: Option<InlineKbMarkup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKbMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUpdResp {
    pub update_id: i64,
    pub message: Option<Message>,
    pub callback_query: Option<CbQuery>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CbQuery {
    pub message: Message,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistMsgWithInlKeyboard {
    pub text: String,
    pub chat_id: i64,
    pub message_id: Option<String>,
    pub reply_markup: InlineKeyboard,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboard {
    pub inline_keyboard: [Vec<InlineKeyboardButton>; 1],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub callback_data: String,
}
