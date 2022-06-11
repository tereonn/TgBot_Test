use anyhow::Result;
use mobc::Pool;
use mobc_redis::redis::aio::Connection;
use mobc_redis::RedisConnectionManager;
use redis::FromRedisValue;
use std::fmt::Debug;

#[derive(Clone)]
pub struct RedisRepo {
    pub pool: Pool<RedisConnectionManager>,
}

impl RedisRepo {
    pub fn new(url: &str) -> Self {
        let client = redis::Client::open(url).expect("Can't connect to redis");
        let manager = RedisConnectionManager::new(client);
        let pool = Pool::builder().max_open(3).max_idle(1).build(manager);

        Self { pool }
    }

    pub async fn read<T>(&self, command: &str, args: Vec<String>) -> Result<Option<T>>
    where
        T: FromRedisValue + Debug,
    {
        let mut con = self.pool.get().await?;

        let mut cmd = redis::cmd(command);
        for a in args {
            cmd.arg(a);
        }

        let result = cmd.query_async(&mut con as &mut Connection).await?;

        Ok(result)
    }

    pub async fn write<T>(&self, command: &str, args: Vec<String>) -> Result<T>
    where
        T: FromRedisValue,
    {
        let mut con = self.pool.get().await?;

        let mut cmd = redis::cmd(command);
        for a in args {
            cmd.arg(a);
        }

        let res = cmd.query_async(&mut con as &mut Connection).await?;

        Ok(res)
    }
}
