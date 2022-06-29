use std::net::TcpListener;
use std::time::Duration;

use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

use crate::configuration::{DatabaseSettings, Settings};
use crate::routes;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let db_pool = get_connection_pool(&configuration.database);
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        println!("{}", address);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = build_server(listener, db_pool)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn start_server(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool(database_settings: &DatabaseSettings) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_settings.with_db());
    Pool::builder()
        .connection_timeout(Duration::from_secs(2))
        .build(manager)
        .expect("Failed to configure a pool.")
}

fn build_server(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .configure(routes::routes)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
