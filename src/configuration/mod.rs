use std::process;

use config::Environment;
use serde::Deserialize;

pub fn get_configuration() -> Configuration {
    let config = config::Config::builder()
        .set_default("http_port", 9101)
        .expect("[CONFIG] cannot set default value for application_port")
        .add_source(config::File::with_name("configuration").required(false))
        .add_source(Environment::default().separator("__"))
        .build()
        .unwrap_or_else(|err| {
            tracing::error!("[CONFIG] Unable to get config builder ({})", err);
            process::exit(1);
        });

    let configuration: Configuration = config.try_deserialize().unwrap_or_else(|err| {
        tracing::error!("[CONFIG] Failed to deserialize config ({})", err);
        process::exit(1);
    });

    configuration
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    username: String,
    password: String,
    port: u16,
    host: String,
    db_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub http_port: u16,
    pub database: DatabaseSettings,
}

impl DatabaseSettings {
    pub fn get_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }

    pub fn get_connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
