use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use std::net::TcpListener;

pub mod configuration;
pub mod routes;
pub mod startup;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/subscriptions", web::post().to(routes::subscribe))
            .route("/health_check", web::get().to(routes::health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
