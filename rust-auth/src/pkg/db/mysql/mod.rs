use config::Config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;
use tokio::sync::OnceCell;

const DEFAULT_KEY: &str = "mysql";

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

#[derive(Debug, Deserialize)]
struct MysqlConfig {
    dsn: String,
}

pub async fn init(c: &Config) {
    let cfg = init_config(c);
    cfg.init_client().await;
}

fn init_config(c: &Config) -> MysqlConfig {
    c.get::<MysqlConfig>(DEFAULT_KEY).unwrap()
}

impl MysqlConfig {
    async fn init_client(self) {
        let conn = Database::connect(ConnectOptions::new(&self.dsn))
            .await
            .unwrap();

        let _ = DB.set(conn);
    }
}

pub fn get_db() -> &'static DatabaseConnection {
    DB.get().unwrap()
}
