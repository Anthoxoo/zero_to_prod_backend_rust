use std::net::TcpListener;
use zero_to_prod_backend_rust::startup::run;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    // reqwest allows us send a request to our own backend.
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert!(response.content_length() == Some(0));
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com"; // body that is valid

    let response = client
        .post(format!("{app_address}/subscription"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send the request POST");

    assert!(response.status() == 200)
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (value, err) in test_cases {
        let response = client
            .post(format!("{app_address}/subscription"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(value)
            .send()
            .await
            .expect("Failed to send the request POST");

        assert!(
            response.status() == 400,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            err
        )
    }
}

// Launch our application in the background
fn spawn_app() -> String {
    //return the port used by the new app.
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind the port using TcpListener");

    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Error binding the ports.");
    // Launch the server as a background task
    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
