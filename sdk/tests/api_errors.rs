use sumup::Client;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn documented_problem_preserves_http_context() {
    let server = MockServer::start().await;
    let raw = r#"{"type":"about:blank","detail":"Access token expired"}"#;
    Mock::given(method("GET"))
        .and(path("/v0.1/checkouts/checkout"))
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_raw(raw, "application/problem+json")
                .insert_header("Retry-After", "10"),
        )
        .mount(&server)
        .await;
    let client = Client::new().with_base_url(server.uri());
    let error = client.checkouts().get("checkout").await.unwrap_err();
    assert_eq!(error.status(), Some(reqwest::StatusCode::UNAUTHORIZED));
    assert_eq!(
        error.problem().unwrap().detail.as_deref(),
        Some("Access token expired")
    );
    assert_eq!(error.headers().unwrap()["retry-after"], "10");
    assert_eq!(error.raw_body().unwrap(), raw.as_bytes());
    assert!(error.body().is_some());
}

#[tokio::test]
async fn malformed_documented_error_remains_an_http_failure() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(401).set_body_raw("{invalid", "application/problem+json"),
        )
        .mount(&server)
        .await;
    let client = Client::new().with_base_url(server.uri());
    let error = client.checkouts().get("checkout").await.unwrap_err();
    assert_eq!(error.status(), Some(reqwest::StatusCode::UNAUTHORIZED));
    assert!(error.problem().is_none());
    assert_eq!(error.raw_body().unwrap(), b"{invalid");
    assert!(matches!(error, sumup::error::SdkError::Unexpected(_)));
}

#[tokio::test]
async fn undocumented_problem_has_typed_details() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(502)
                .set_body_json(serde_json::json!({"type": "about:blank", "detail": "Unavailable"})),
        )
        .mount(&server)
        .await;
    let client = Client::new().with_base_url(server.uri());
    let error = client.checkouts().get("checkout").await.unwrap_err();
    assert_eq!(error.status(), Some(reqwest::StatusCode::BAD_GATEWAY));
    assert_eq!(
        error.problem().unwrap().detail.as_deref(),
        Some("Unavailable")
    );
}
