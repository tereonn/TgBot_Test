use crate::{db::RedisRepo, dto::Message, helpers::prop_gen::*};
use anyhow::Result;

pub struct HandleCommonMsgRedis {
    repo: RedisRepo,
}
impl HandleCommonMsgRedis {
    pub fn new(repo: RedisRepo) -> Self {
        Self { repo }
    }

    pub async fn save_msg(&self, msg: Message) -> Result<Option<()>> {
        let id: i64 = msg.chat.id;
        let new_id: String = self
            .repo
            .write(
                "XADD",
                [
                    get_chat_msgs_p_name(id),
                    "*".to_string(),
                    "text".to_string(),
                    msg.text.unwrap(),
                ]
                .to_vec(),
            )
            .await?;

        self.repo
            .write("SET", [get_chat_last_hist_msg_p_name(id), new_id].to_vec())
            .await?;

        self.repo
            .write(
                "SET",
                [get_chat_last_msg_p_name(id), msg.message_id.to_string()].to_vec(),
            )
            .await?;

        Ok(None)
    }
}
