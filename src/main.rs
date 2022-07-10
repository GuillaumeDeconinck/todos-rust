use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use todos_api::{startup::run, telemetry, tools::get_configuration};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_tracing_subscriber("zero2prod".into(), "info".into());
    telemetry::init_tracing_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to get DB connection");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
