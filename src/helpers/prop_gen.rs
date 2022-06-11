pub fn get_chat_msgs_p_name(id: i64) -> String {
    format!("msgs::{}", id)
}

pub fn get_chat_last_hist_msg_p_name(id: i64) -> String {
    format!("hist::{}", id)
}

pub fn get_chat_last_msg_p_name(id: i64) -> String {
    format!("last_msg::{}", id)
}

pub fn get_chat_last_hist_p_name(id: i64) -> String {
    format!("last_hist_msg::{}", id)
}
