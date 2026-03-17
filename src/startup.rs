use crate::routes::*;
use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check)) // After the path, we specify the type of request we are looking for.
            .route("/subscription", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
