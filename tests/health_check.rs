use std::net::TcpListener;

#[tokio::test]
async fn health_check_workd() {
    let app = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listiner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listiner.local_addr().unwrap().port();

    let server = zero2prod::run(listiner).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
