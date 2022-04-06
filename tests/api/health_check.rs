use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_workd() { // 18.30 seconds
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Diagnosing Slow Tests 
// CURRENT => 18.30 seconds
// #TRY1 => `base.yml` timeout to 1
//      RESULT => 8.20 seconds
// #TRY2 => `base.yml` timeout to 1000000
//      RESULT => 8.24 seconds
// #TRY3 => `base.yml` timeout to 10000
//      RESULT => 156.98 sec, 7.87 sec
