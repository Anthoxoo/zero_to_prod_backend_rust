use std::net::TcpListener;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, web};

async fn health_check() -> HttpResponse {
    return HttpResponse::Ok().finish();
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    /*Form here allows us to place informations for us in the FormData struct using the "application/x-www-form-urlencoded" method
     * that looks like this : "name=le%20guin&email=ursula_le_guin%40gmail.com"
     * so the name and email will automaticaly go in the right field */
    return HttpResponse::Ok().finish(); // TODO!
}

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
