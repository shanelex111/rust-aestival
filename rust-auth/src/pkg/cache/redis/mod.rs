use config::Config;
use redis::cluster::ClusterClient;
use redis::cluster_async::ClusterConnection;
use serde::Deserialize;
use tokio::sync::OnceCell;

const DEFAULT_KEY: &str = "redis";

static RDB: OnceCell<ClusterConnection> = OnceCell::const_new();

#[derive(Debug, Deserialize)]
struct RedisConfig {
    addrs: Vec<String>,
}

pub async fn init(c: &Config) {
    let cfg = init_config(c);
    cfg.init_client().await;
}
fn init_config(c: &Config) -> RedisConfig {
    c.get::<RedisConfig>(DEFAULT_KEY).unwrap()
}
impl RedisConfig {
    async fn init_client(self) {
        let client = ClusterClient::new(self.addrs).unwrap();
        let conn = client.get_async_connection().await.unwrap();
        let _ = RDB.set(conn);
    }
}

pub fn get_db() -> &'static ClusterConnection {
    RDB.get().unwrap()
}
