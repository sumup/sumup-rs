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

#[tokio::test]
async fn interrupted_error_bodies_preserve_http_context() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    for status in [401, 502] {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut stream = BufReader::new(stream);
            loop {
                let mut line = String::new();
                assert!(stream.read_line(&mut line).await.unwrap() > 0);
                if line == "\r\n" {
                    break;
                }
            }
            // The declared body length is deliberately larger than the bytes sent.
            let response = format!(
                "HTTP/1.1 {status} Error\r\nContent-Length: 100\r\nRetry-After: 7\r\nConnection: close\r\n\r\nbad"
            );
            stream
                .get_mut()
                .write_all(response.as_bytes())
                .await
                .unwrap();
        });
        let client = Client::new().with_base_url(format!("http://{address}"));
        let error = client.checkouts().get("checkout").await.unwrap_err();
        server.await.unwrap();
        assert_eq!(error.status().unwrap().as_u16(), status);
        assert_eq!(error.headers().unwrap()["retry-after"], "7");
        assert!(matches!(error, sumup::SdkError::Response(_)));
        assert!(std::error::Error::source(&error).is_some());
        assert!(error.problem().is_none());
        assert!(error.raw_body().is_none());
    }
}

#[tokio::test]
async fn documented_error_without_a_body_schema_exposes_problem_details() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v0.1/customers"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "type": "about:blank", "detail": "Invalid customer"
        })))
        .mount(&server)
        .await;
    let client = Client::new().with_base_url(server.uri());
    let error = client
        .customers()
        .create(sumup::resources::customers::CreateRequest {
            customer_id: "customer".into(),
            personal_details: None,
        })
        .await
        .unwrap_err();
    assert_eq!(
        error.problem().unwrap().detail.as_deref(),
        Some("Invalid customer")
    );
    assert_eq!(error.status(), Some(reqwest::StatusCode::BAD_REQUEST));
    assert!(error.body().is_some());
}
