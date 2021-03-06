use crate::helpers::spawn_app;

use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/send"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app.post_sub(body.into()).await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_persists_the_new_subscriber() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Act
    app.post_sub(body.into()).await;

    // Assert
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursela_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (bad_body, error_message) in test_cases {
        let response = app.post_sub(bad_body.into()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API didn't fail with 400 Bad Request when the payload was: {}",
            error_message
        )
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = app.post_sub(body.into()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "This API did not return a 400 Bad Request when the playload was {}.",
            description
        );
    }
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/send"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_sub(body.into()).await;
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/send"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // println!("APP POST SUB AWAIT{:?}",
    app.post_sub(body.into()).await;
    // );

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_link = app.get_confirmation_links(&email_request);

    // println!("EMAIL REQUEST: {}", email_request);

    // let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();
    // // println!("BODY: {}", body);
    // let get_link = |s: &str| {
    //     let links: Vec<_> = linkify::LinkFinder::new()
    //         .links(s)
    //         .filter(|l| *l.kind() == linkify::LinkKind::Url)
    //         .collect();
    //     println!("Links: {:?}", links);
    //     assert_eq!(links.len(), 1);
    //     links[0].as_str().to_owned()
    // };
    // // See structre in email_client.rs, [0]: text/html and [1]: text/plain
    // let html_link = get_link(
    //     &body.get("content").unwrap()[0]
    //         .get("value")
    //         .unwrap()
    //         .as_str()
    //         .unwrap(),
    // );
    // let text_link = get_link(
    //     &body.get("content").unwrap()[1]
    //         .get("value")
    //         .unwrap()
    //         .as_str()
    //         .unwrap(),
    // );

    assert_eq!(confirmation_link.html, confirmation_link.plain_text);
}
