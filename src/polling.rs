use anyhow::Result;
use futures::future::join_all;
use std::sync::Arc;

use crate::{
    connector::TgConnector,
    general_handler::handle_msg,
    helpers::parser::parse_resp,
    types::{PollingCommand, UpdateData},
};

pub async fn make_polling(con: Arc<TgConnector>) -> Result<()> {
    let mut offs_idx: i64 = 0;
    while let PollingCommand::Continue(data) = con.get_update_msg(offs_idx).await? {
        match data {
            UpdateData::Data(d) => {
                if d.len() == 0 {
                    continue;
                }

                let last_upd_idx = d.last().unwrap().update_id;
                offs_idx = last_upd_idx + 1;

                let mut tasks = Vec::new();
                for resp in d.into_iter() {
                    let parsed = parse_resp(resp);
                    tasks.push(handle_msg(&con, parsed));
                }

                let res = join_all(tasks).await;

                for r in res {
                    match r {
                        Err(e) => eprintln!("{e}"),
                        Ok(v) => match v {
                            Err(e) => eprintln!("{e}"),
                            _ => (),
                        },
                    }
                }
            }
            _ => (),
        }
    }

    Ok(())
}
