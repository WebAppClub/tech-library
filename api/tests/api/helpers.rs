use std::time::Duration;

use diesel::r2d2::{self, ConnectionManager, ManageConnection};
use diesel::{PgConnection, RunQueryDsl};
use uuid::Uuid;

use api::configuration::{get_configuration, DatabaseSettings};
use api::startup::{get_connection_pool, Application, PgPool};

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // テストケースごとに異なるデータベースを使用する
        c.database.database_name = Uuid::new_v4().to_string();
        // OSによって使用可能なポートがランダムで割り当てられる
        c.application.port = 0;
        c
    };
    // データベースの create と migrate
    configure_database(&configuration.database);

    // バックグラウンドタスクとしてアプリケーションを起動
    let application =
        Application::build(configuration.clone()).expect("Failed to build application.");
    let application_port = application.port();
    let _ = tokio::spawn(application.start_server());
    TestApp {
        address: format!("http://127.0.0.1:{application_port}"),
        port: application_port,
        db_pool: get_connection_pool(&configuration.database),
    }
}

fn configure_database(database_settings: &DatabaseSettings) -> PgPool {
    // Create();
    let connection = ConnectionManager::<PgConnection>::new(database_settings.without_db())
        .connect()
        .expect("Failed to create a new connection.");
    diesel::sql_query(format!(
        r#"
    CREATE DATABASE "{}";
            "#,
        database_settings.database_name
    ))
    .execute(&connection)
    .expect("Failed to create database.");

    // Migrate
    let manager = ConnectionManager::<PgConnection>::new(database_settings.with_db());
    let pool = r2d2::Pool::builder()
        .connection_timeout(Duration::from_secs(2))
        .build(manager)
        .expect("Failed to create pool.");
    let connection_pool = pool
        .get()
        .expect("Failed to retrieves a connection from the pool.");
    diesel_migrations::run_pending_migrations(&connection_pool)
        .expect("Failed to run all migrations that have not yet been run.");

    pool
}
