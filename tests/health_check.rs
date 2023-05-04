use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    //Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    //Act
    let response = client
        //Use application to get address
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    //Asert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

//Launch our stuff
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
    // we retrive the port assigned to us
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application to the port
    format!("http://127.0.0.1:{}", port)
}
