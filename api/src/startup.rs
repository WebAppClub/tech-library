use std::net::{SocketAddr, TcpListener};

use actix_web::dev::Server;
use actix_web::{App, HttpServer};

use crate::routes;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub fn build(host: [u8; 4], port: u16) -> Result<Self, std::io::Error> {
        let address = SocketAddr::from((host, port));
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = start_server(listener)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn start_server(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn start_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || App::new().configure(routes::routes))
        .listen(listener)?
        .run();
    Ok(server)
}
