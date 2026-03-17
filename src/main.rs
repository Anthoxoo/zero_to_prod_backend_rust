use std::net::TcpListener;
use zero_to_prod_backend_rust::configuration::get_configuration;
use zero_to_prod_backend_rust::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let _configuration = get_configuration()
        .expect("Error while getting the configuration from the configuration.yaml file.");

    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind the port using TcpListener");
    run(listener).expect("Error binding the ports.").await
}
