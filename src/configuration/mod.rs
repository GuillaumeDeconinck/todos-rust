use std::process;

use config::Environment;
use lazy_static::lazy_static;
use serde::Deserialize;

// Easiest way to have env vars accessible everywhere without having to "package" it in an Actix data cell
// I'm not convinced at 100%, but it works
lazy_static! {
    // # Structure containing the env vars and their values
    pub static ref SETTINGS: Configuration = {
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
    };
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    username: String,
    password: String,
    port: u16,
    host: String,
    database_name: String,
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
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn get_connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
