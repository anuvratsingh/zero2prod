use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::spawn_app;

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
async fn subscriber_sends_a_confirmation_email_for_valid_data() {
    let app = spawn_app().await;
    let body = "name=le%20guin%email=ursula_le_guin%40gmail.com";

    Mock::given(path("/send"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    dbg!(app.post_sub(body.into()).await);
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    let app = spawn_app().await;
    let body = "name=le%20guin%email=ursula_le_guin%40gmail.com";

    Mock::given(path("/send"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    dbg!(app.post_sub(body.into()).await);

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    dbg!(email_request);

    let confirmation_links = app.get_confirmation_links(email_request);

    // !!!! Send grid doesn't have HTML, it has but skipping it as of now !!!!

    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}
