use std::process::Command;
use std::time::Duration;

use serde_json::json;
use sumup::{Authorization, Client, version};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ENV_TEST_CASE: &str = "SUMUP_RS_ENV_TEST_CASE";

fn run_environment_test(case: &str, api_key: Option<&str>) {
    let mut command = Command::new(std::env::current_exe().expect("resolve current test binary"));
    command
        .args(["--exact", "client_environment_case"])
        .env(ENV_TEST_CASE, case);

    if let Some(api_key) = api_key {
        command.env("SUMUP_API_KEY", api_key);
    } else {
        command.env_remove("SUMUP_API_KEY");
    }

    let output = command.output().expect("run environment test subprocess");
    assert!(
        output.status.success(),
        "environment test subprocess failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn client_environment_case() {
    let Ok(case) = std::env::var(ENV_TEST_CASE) else {
        return;
    };

    let client = Client::new();
    match case.as_str() {
        "reads_authorization" => assert_eq!(client.authorization(), Some("env-token")),
        "overrides_authorization" => assert_eq!(
            client
                .with_authorization(Authorization::api_key("override-token"))
                .authorization(),
            Some("override-token")
        ),
        "missing_authorization" => assert!(client.authorization().is_none()),
        _ => panic!("unknown environment test case: {case}"),
    }
}

#[test]
fn client_uses_default_base_url() {
    let client = Client::default();
    assert_eq!(client.base_url(), "https://api.sumup.com");
}

#[test]
fn client_with_base_url_overrides_default() {
    let client = Client::new().with_base_url("https://mock.sumup.internal.test");
    assert_eq!(client.base_url(), "https://mock.sumup.internal.test");
}

#[test]
fn client_with_timeout_updates_timeout() {
    let timeout = Duration::from_secs(42);
    let client = Client::new().with_timeout(timeout);
    assert_eq!(client.timeout(), timeout);
}

#[test]
fn client_reads_authorization_from_env() {
    run_environment_test("reads_authorization", Some("env-token"));
}

#[test]
fn client_with_authorization_overrides_env_value() {
    run_environment_test("overrides_authorization", Some("env-token"));
}

#[tokio::test]
async fn client_requests_include_user_agent_and_custom_authorization() {
    let server = MockServer::start().await;
    let override_token = "override-token";
    let expected_auth = format!("Bearer {}", override_token);
    let expected_user_agent = version::user_agent();

    let _mock = Mock::given(method("GET"))
        .and(path("/v0.1/checkouts"))
        .and(header("User-Agent", expected_user_agent.as_str()))
        .and(header("Authorization", expected_auth.as_str()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .expect(1)
        .mount_as_scoped(&server)
        .await;

    let client = Client::new()
        .with_base_url(server.uri())
        .with_authorization(Authorization::api_key(override_token));

    client
        .checkouts()
        .list(sumup::resources::checkouts::ListParams::default())
        .await
        .expect("request should succeed");
}

#[tokio::test]
async fn client_requests_include_runtime_headers() {
    let server = MockServer::start().await;
    let expected_user_agent = version::user_agent();

    let mut mock = Mock::given(method("GET"))
        .and(path("/v0.1/checkouts"))
        .and(header("User-Agent", expected_user_agent.as_str()));

    for (header_name, header_value) in version::runtime_info() {
        mock = mock.and(header(header_name, header_value.as_str()));
    }

    let _mock = mock
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .expect(1)
        .mount_as_scoped(&server)
        .await;

    let client = Client::new().with_base_url(server.uri());

    client
        .checkouts()
        .list(sumup::resources::checkouts::ListParams::default())
        .await
        .expect("request should succeed");
}

#[test]
fn client_returns_none_when_authorization_missing() {
    run_environment_test("missing_authorization", None);
}
