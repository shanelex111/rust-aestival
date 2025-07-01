mod pkg;

use crate::pkg::cache::redis;
use crate::pkg::config;
use crate::pkg::db::mysql;
use ::redis::AsyncTypedCommands;

#[tokio::main]
async fn main() {
    // 1. load config - 加载配置文件
    let c = config::load();

    // 2. init server components - 初始化组件
    mysql::init(&c).await;

    redis::init(&c).await;
    // 3. load config configs - 加载业务配置

    redis::get_db()
        .clone()
        .set("rust-dev", "hello word")
        .await
        .unwrap();
    // 4. services run - 业务启动
}
