use config::Config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;
use tokio::sync::OnceCell;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();
#[derive(Debug, Deserialize)]
struct MysqlConfig {
    dsn: String,
}
const DEFAULT_KEY: &str = "mysql";

pub async fn init(c: &mut Config) {
    let mc = init_config(c);
    mc.init_client().await;
}

fn init_config(c: &mut Config) -> MysqlConfig {
    c.get::<MysqlConfig>(DEFAULT_KEY).unwrap()
}

impl MysqlConfig {
    async fn init_client(&self) {
        let conn = Database::connect(ConnectOptions::new(&self.dsn))
            .await
            .unwrap();

        DB.set(conn).unwrap();
    }
}
