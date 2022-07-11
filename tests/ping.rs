use std::net::TcpListener;

use once_cell::sync::Lazy;
use sqlx::PgPool;
use todos_api::{
    configuration::{DatabaseSettings, SETTINGS},
    startup, telemetry,
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = telemetry::get_tracing_subscriber("test".into(), "info".into());
    telemetry::init_tracing_subscriber(subscriber);
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let pool = configure_database(&SETTINGS.database).await;

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = startup::run(listener, pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);
    return TestApp {
        address,
        db_pool: pool,
    };
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let pool = PgPool::connect(&config.get_connection_string())
        .await
        .expect("Failed to connect to generated DB");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to execute migrations");

    pool
}

#[tokio::test]
async fn ping_works() {
    // Setup
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Test
    let response = client
        .get(&format!("{}/ping", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    // Check
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
