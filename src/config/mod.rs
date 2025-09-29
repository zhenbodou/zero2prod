use crate::config::{application::ApplicationSettings, database::DatabaseSettings};
use serde::Serialize;

pub mod application;
pub mod database;

#[derive(serde::Deserialize, Serialize, Debug, Clone)]
pub struct Settings {
    #[serde(default)]
    pub database: DatabaseSettings,
    #[serde(default)]
    pub application: ApplicationSettings,
}

impl Settings {
    pub fn configuration() -> anyhow::Result<Self> {
        let setting_builder = config::Config::builder()
            .add_source(config::File::with_name("./config/application").required(true))
            .add_source(config::File::with_name("./config/application-dev").required(false))
            .add_source(config::File::with_name("./config/application-prod").required(false));
        let settings: Self = setting_builder.build()?.try_deserialize()?;
        Ok(settings)
    }

    pub fn application_settings(&self) -> &ApplicationSettings {
        &self.application
    }

    pub fn database_settings(&self) -> &DatabaseSettings {
        &self.database
    }
}
