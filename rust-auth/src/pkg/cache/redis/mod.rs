use config::Config;
use redis::cluster::ClusterClient;
use redis::cluster_async::ClusterConnection;
use serde::Deserialize;
use tokio::sync::OnceCell;

pub static RDB: OnceCell<ClusterConnection> = OnceCell::const_new();
const DEFAULT_KEY: &str = "redis";
#[derive(Debug, Deserialize)]
struct RedisConfig {
    addrs: Vec<String>,
}

pub async fn init(c: &mut Config) {
    let cfg = init_config(c);
    cfg.init_client().await;
}
fn init_config(c: &mut Config) -> RedisConfig {
    c.get::<RedisConfig>(DEFAULT_KEY).unwrap()
}
impl RedisConfig {
    async fn init_client(self) {
        let client = ClusterClient::new(self.addrs).unwrap();
        let conn = client.get_async_connection().await.unwrap();
        RDB.set(conn);
    }
}
