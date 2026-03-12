use std::net::TcpListener;

use zero_to_prod_backend_rust::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind the port using TcpListener");
    run(listener).expect("Error binding the ports.").await
}
