mod pkg;

use crate::pkg::config;
use crate::pkg::db::mysql;
use crate::pkg::cache::redis;
#[tokio::main]
async fn main() {
    // 1. load config - 加载配置文件
    let mut c = config::load();

    // 2. init server components - 初始化组件
    mysql::init(&mut c).await;

    redis::init(&mut c).await;
    // 3. load config configs - 加载业务配置

    // 4. services run - 业务启动
}
