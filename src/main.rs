use actix_web::{App, HttpRequest, HttpServer, web};

async fn greet(request: HttpRequest) -> String {
    let name = request.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
