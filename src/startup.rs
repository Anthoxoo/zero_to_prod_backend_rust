use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check)) // After the path, we specify the type of request we are looking for.
            .route("/subscription", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
