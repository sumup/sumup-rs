use std::time::Duration;

use serde_json::json;
use serial_test::serial;
use sumup::{version, Client};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

struct EnvVarGuard {
    key: &'static str,
    original: Option<String>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let original = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, original }
    }

    fn unset(key: &'static str) -> Self {
        let original = std::env::var(key).ok();
        std::env::remove_var(key);
        Self { key, original }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(ref value) = self.original {
            std::env::set_var(self.key, value);
        } else {
            std::env::remove_var(self.key);
        }
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
#[serial]
fn client_reads_authorization_from_env() {
    let token = "env-token";
    let _guard = EnvVarGuard::set("SUMUP_API_KEY", token);

    let client = Client::new();

    assert_eq!(client.authorization_token(), Some(token));
}

#[test]
#[serial]
fn client_with_authorization_overrides_env_value() {
    let _guard = EnvVarGuard::set("SUMUP_API_KEY", "env-token");
    let override_token = "override-token";

    let client = Client::new().with_authorization(override_token);

    assert_eq!(client.authorization_token(), Some(override_token));
}

#[tokio::test]
#[serial]
async fn client_requests_include_user_agent_and_custom_authorization() {
    let server = MockServer::start().await;
    let _guard = EnvVarGuard::set("SUMUP_API_KEY", "env-token");
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
        .with_authorization(override_token);

    client
        .checkouts()
        .list(sumup::resources::checkouts::ListParams::default())
        .await
        .expect("request should succeed");
}

#[tokio::test]
#[serial]
async fn client_requests_include_runtime_headers() {
    let server = MockServer::start().await;
    let _guard = EnvVarGuard::set("SUMUP_API_KEY", "env-token");
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
#[serial]
fn client_returns_none_when_authorization_missing() {
    let _guard = EnvVarGuard::unset("SUMUP_API_KEY");
    let client = Client::new();
    assert!(client.authorization_token().is_none());
}
