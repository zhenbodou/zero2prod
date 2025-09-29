use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 5432,
            username: "postgres".into(),
            password: "password".into(),
            database_name: "zero2prod".into(),
        }
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub async fn get_database_connection(&self) -> anyhow::Result<DatabaseConnection> {
        let mut opt = ConnectOptions::new(self.connection_string());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false); // Setting default PostgreSQL schema

        let db = Database::connect(opt).await?;
        Ok(db)
    }
}
