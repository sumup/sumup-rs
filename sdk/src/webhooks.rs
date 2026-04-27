//! Helpers for verifying and parsing SumUp webhook deliveries.
//!
//! The current webhook service signs `v1:<unix_timestamp>:<raw_body>` with
//! HMAC-SHA256 and sends the lowercase hex digest in the
//! `X-SumUp-Webhook-Signature` header together with the Unix timestamp in the
//! `X-SumUp-Webhook-Timestamp` header. These helpers verify both headers before
//! deserializing the payload.
//!
//! The webhook payload is intentionally thin, so the client parses a typed
//! notification and lets callers fetch the referenced API object when needed.
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// Header carrying the versioned webhook payload signature.
pub const SIGNATURE_HEADER: &str = "X-SumUp-Webhook-Signature";

/// Header carrying the Unix timestamp used for signing.
pub const TIMESTAMP_HEADER: &str = "X-SumUp-Webhook-Timestamp";

/// Current webhook signature scheme version.
pub const SIGNATURE_VERSION: &str = "v1";

/// Default maximum allowed difference between the webhook timestamp and the
/// local clock.
pub const DEFAULT_TOLERANCE: Duration = Duration::from_secs(5 * 60);

/// Errors returned by webhook helpers.
#[derive(Debug)]
pub enum WebhookError {
    /// The signature header was missing.
    MissingSignature,
    /// The timestamp header was missing.
    MissingTimestamp,
    /// The signature header was malformed.
    InvalidSignatureHeader,
    /// The timestamp header was not a valid Unix timestamp.
    InvalidTimestampHeader(std::num::ParseIntError),
    /// The signature did not match the payload.
    InvalidSignature,
    /// The webhook timestamp was outside the configured tolerance window.
    SignatureExpired,
    /// The body was not valid JSON for the expected webhook shape.
    InvalidPayload(serde_json::Error),
    /// The event type and object type combination did not match a known schema.
    UnexpectedObjectType {
        event_type: String,
        object_type: String,
    },
}

impl std::fmt::Display for WebhookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSignature => write!(f, "missing webhook signature header"),
            Self::MissingTimestamp => write!(f, "missing webhook timestamp header"),
            Self::InvalidSignatureHeader => write!(f, "invalid webhook signature header"),
            Self::InvalidTimestampHeader(err) => {
                write!(f, "invalid webhook timestamp header: {}", err)
            }
            Self::InvalidSignature => write!(f, "invalid webhook signature"),
            Self::SignatureExpired => write!(f, "webhook timestamp outside allowed tolerance"),
            Self::InvalidPayload(err) => write!(f, "invalid webhook payload: {}", err),
            Self::UnexpectedObjectType {
                event_type,
                object_type,
            } => write!(
                f,
                "unexpected object type '{}' for event '{}'",
                object_type, event_type
            ),
        }
    }
}

impl std::error::Error for WebhookError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidTimestampHeader(err) => Some(err),
            Self::InvalidPayload(err) => Some(err),
            Self::MissingSignature
            | Self::MissingTimestamp
            | Self::InvalidSignatureHeader
            | Self::InvalidSignature
            | Self::SignatureExpired
            | Self::UnexpectedObjectType { .. } => None,
        }
    }
}

/// Errors returned when resolving a webhook object from the API.
#[derive(Debug)]
pub enum WebhookFetchError {
    /// The webhook object URL was not a valid absolute URL.
    InvalidObjectUrl(String),
    /// The API request failed or returned a non-success response.
    Sdk(crate::error::SdkError),
}

impl From<crate::error::SdkError> for WebhookFetchError {
    fn from(value: crate::error::SdkError) -> Self {
        Self::Sdk(value)
    }
}

impl std::fmt::Display for WebhookFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidObjectUrl(url) => write!(f, "invalid webhook object url: {}", url),
            Self::Sdk(err) => write!(f, "failed to fetch webhook object: {}", err),
        }
    }
}

impl std::error::Error for WebhookFetchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Sdk(err) => Some(err),
            Self::InvalidObjectUrl(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct RawObject {
    id: String,
    #[serde(rename = "type")]
    object_type: String,
    url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct RawEvent {
    id: String,
    #[serde(rename = "type")]
    event_type: String,
    created_at: crate::datetime::DateTime,
    object: RawObject,
}

/// Generic webhook object reference.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Object {
    /// Resource identifier.
    pub id: String,
    /// Resource type emitted by the webhook service.
    #[serde(rename = "type")]
    pub object_type: String,
    /// Canonical API URL for the resource.
    pub url: String,
}

impl From<RawObject> for Object {
    fn from(value: RawObject) -> Self {
        Self {
            id: value.id,
            object_type: value.object_type,
            url: value.url,
        }
    }
}

/// Trait implemented by typed webhook events that can resolve their referenced
/// API object.
pub trait FetchObject {
    /// The concrete SDK object returned for this event.
    type Object;

    /// Fetches the latest representation of the referenced API object.
    fn fetch_object(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Object, WebhookFetchError>> + '_;
}

/// Reusable webhook helper bound to a client and signing secret.
#[derive(Debug, Clone)]
pub struct WebhookHandler<'a> {
    client: &'a crate::Client,
    secret: Vec<u8>,
    tolerance: Duration,
}

impl<'a> WebhookHandler<'a> {
    /// Creates a webhook handler bound to a client and signing secret.
    pub fn new(client: &'a crate::Client, secret: impl AsRef<[u8]>) -> Self {
        Self {
            client,
            secret: secret.as_ref().to_vec(),
            tolerance: DEFAULT_TOLERANCE,
        }
    }

    /// Returns the client bound to this handler.
    pub fn client(&self) -> &'a crate::Client {
        self.client
    }

    /// Overrides the allowed clock skew for webhook signature verification.
    pub fn with_tolerance(mut self, tolerance: Duration) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Verifies a webhook payload signature against the raw body.
    pub fn verify(
        &self,
        payload: &[u8],
        signature_header: impl AsRef<str>,
        timestamp_header: impl AsRef<str>,
    ) -> Result<(), WebhookError> {
        verify_signature_with_tolerance(
            &self.secret,
            payload,
            signature_header,
            timestamp_header,
            self.tolerance,
        )
    }

    /// Parses a raw webhook notification into a typed event bound to the client.
    pub fn parse(&self, payload: &[u8]) -> Result<EventNotification<'a>, WebhookError> {
        parse_event_notification(self.client, payload)
    }

    /// Verifies a webhook signature and parses the notification into a typed event.
    pub fn verify_and_parse(
        &self,
        payload: &[u8],
        signature_header: impl AsRef<str>,
        timestamp_header: impl AsRef<str>,
    ) -> Result<EventNotification<'a>, WebhookError> {
        self.verify(payload, signature_header, timestamp_header)?;
        self.parse(payload)
    }
}

async fn fetch_object<T>(client: &crate::Client, object_url: &str) -> Result<T, WebhookFetchError>
where
    T: serde::de::DeserializeOwned,
{
    reqwest::Url::parse(object_url)
        .map_err(|_| WebhookFetchError::InvalidObjectUrl(object_url.to_owned()))?;
    client.get(object_url).await.map_err(Into::into)
}

macro_rules! define_events {
    (
        $(
            $variant:ident => {
                event: $event_struct:ident,
                event_type: $event_type:literal,
                object_type: $expected_object_type:literal,
                fetched_object: $fetched_object:path,
                object_doc: $object_doc:literal
            }
        ),+ $(,)?
    ) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $event_struct<'a> {
                /// Event identifier.
                pub id: String,
                /// Event creation timestamp.
                pub created_at: crate::datetime::DateTime,
                #[doc = $object_doc]
                pub object: Object,
                client: &'a crate::Client,
            }

            impl<'a> $event_struct<'a> {
                fn from_raw(client: &'a crate::Client, event: RawEvent) -> Result<Self, WebhookError> {
                    if event.object.object_type != $expected_object_type {
                        return Err(WebhookError::UnexpectedObjectType {
                            event_type: event.event_type,
                            object_type: event.object.object_type,
                        });
                    }

                    Ok(Self {
                        id: event.id,
                        created_at: event.created_at,
                        object: event.object.into(),
                        client,
                    })
                }

                /// Returns the webhook event type.
                pub fn event_type(&self) -> &'static str {
                    $event_type
                }
            }

            impl<'a> FetchObject for $event_struct<'a> {
                type Object = $fetched_object;

                fn fetch_object(
                    &self,
                ) -> impl std::future::Future<Output = Result<Self::Object, WebhookFetchError>> + '_ {
                    fetch_object(self.client, &self.object.url)
                }
            }
        )+

        /// Typed webhook notifications known by the SDK.
        #[derive(Debug, Clone)]
        pub enum EventNotification<'a> {
            $(
                $variant($event_struct<'a>),
            )+
            Unknown(UnknownEvent<'a>),
        }

        impl<'a> EventNotification<'a> {
            /// Returns the webhook event type.
            pub fn event_type(&self) -> &str {
                match self {
                    $(
                        Self::$variant(event) => event.event_type(),
                    )+
                    Self::Unknown(event) => &event.event_type,
                }
            }

            /// Returns `true` when resolving the thin notification into the full
            /// API resource is usually the most useful next step.
            pub fn should_fetch_object(&self) -> bool {
                match self {
                    $(
                        Self::$variant(_) => true,
                    )+
                    Self::Unknown(_) => false,
                }
            }
        }

        fn parse_known_event<'a>(
            client: &'a crate::Client,
            event: RawEvent,
        ) -> Result<EventNotification<'a>, WebhookError> {
            match event.event_type.as_str() {
                $(
                    $event_type => Ok(EventNotification::$variant($event_struct::from_raw(client, event)?)),
                )+
                _ => Ok(EventNotification::Unknown(UnknownEvent::from_raw(client, event))),
            }
        }
    };
}

define_events! {
    CheckoutCreated => {
        event: CheckoutCreatedEvent,
        event_type: "checkout.created",
        object_type: "checkout",
        fetched_object: crate::resources::checkouts::CheckoutSuccess,
        object_doc: "Checkout reference carried by the webhook."
    },
    CheckoutUpdated => {
        event: CheckoutUpdatedEvent,
        event_type: "checkout.updated",
        object_type: "checkout",
        fetched_object: crate::resources::checkouts::CheckoutSuccess,
        object_doc: "Checkout reference carried by the webhook."
    },
    CheckoutPaid => {
        event: CheckoutPaidEvent,
        event_type: "checkout.paid",
        object_type: "checkout",
        fetched_object: crate::resources::checkouts::CheckoutSuccess,
        object_doc: "Checkout reference carried by the webhook."
    },
    CheckoutFailed => {
        event: CheckoutFailedEvent,
        event_type: "checkout.failed",
        object_type: "checkout",
        fetched_object: crate::resources::checkouts::CheckoutSuccess,
        object_doc: "Checkout reference carried by the webhook."
    },
    CheckoutExpired => {
        event: CheckoutExpiredEvent,
        event_type: "checkout.expired",
        object_type: "checkout",
        fetched_object: crate::resources::checkouts::CheckoutSuccess,
        object_doc: "Checkout reference carried by the webhook."
    },
    MemberCreated => {
        event: MemberCreatedEvent,
        event_type: "member.created",
        object_type: "member",
        fetched_object: crate::resources::members::Member,
        object_doc: "Member reference carried by the webhook."
    },
    MemberUpdated => {
        event: MemberUpdatedEvent,
        event_type: "member.updated",
        object_type: "member",
        fetched_object: crate::resources::members::Member,
        object_doc: "Member reference carried by the webhook."
    }
}

/// Event type used when the SDK does not yet know the webhook schema.
#[derive(Debug, Clone)]
pub struct UnknownEvent<'a> {
    /// Event identifier.
    pub id: String,
    /// Raw event type emitted by the webhook service.
    pub event_type: String,
    /// Event creation timestamp.
    pub created_at: crate::datetime::DateTime,
    /// Generic object reference carried by the webhook.
    pub object: Object,
    _client: &'a crate::Client,
}

impl<'a> UnknownEvent<'a> {
    fn from_raw(client: &'a crate::Client, event: RawEvent) -> Self {
        Self {
            id: event.id,
            event_type: event.event_type,
            created_at: event.created_at,
            object: event.object.into(),
            _client: client,
        }
    }
}

impl crate::Client {
    /// Parses a raw webhook notification into a typed event bound to this
    /// client, allowing follow-up `fetch_object()` calls.
    pub fn parse_event_notification<'a>(
        &'a self,
        payload: &[u8],
    ) -> Result<EventNotification<'a>, WebhookError> {
        parse_event_notification(self, payload)
    }

    /// Verifies a webhook signature and parses the notification into a typed
    /// event bound to this client.
    pub fn verify_and_parse_event_notification<'a>(
        &'a self,
        secret: impl AsRef<[u8]>,
        payload: &[u8],
        signature_header: impl AsRef<str>,
        timestamp_header: impl AsRef<str>,
    ) -> Result<EventNotification<'a>, WebhookError> {
        self.webhook_handler(secret)
            .verify_and_parse(payload, signature_header, timestamp_header)
    }
}

/// Verifies a webhook payload signature against the raw body.
pub fn verify_signature(
    secret: impl AsRef<[u8]>,
    payload: &[u8],
    signature_header: impl AsRef<str>,
    timestamp_header: impl AsRef<str>,
) -> Result<(), WebhookError> {
    verify_signature_with_tolerance(
        secret,
        payload,
        signature_header,
        timestamp_header,
        DEFAULT_TOLERANCE,
    )
}

fn verify_signature_with_tolerance(
    secret: impl AsRef<[u8]>,
    payload: &[u8],
    signature_header: impl AsRef<str>,
    timestamp_header: impl AsRef<str>,
    tolerance: Duration,
) -> Result<(), WebhookError> {
    let signature = signature_header.as_ref().trim();
    if signature.is_empty() {
        return Err(WebhookError::MissingSignature);
    }

    let timestamp = timestamp_header.as_ref().trim();
    if timestamp.is_empty() {
        return Err(WebhookError::MissingTimestamp);
    }
    let timestamp = timestamp
        .parse::<u64>()
        .map_err(WebhookError::InvalidTimestampHeader)?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_secs();
    if now.abs_diff(timestamp) > tolerance.as_secs() {
        return Err(WebhookError::SignatureExpired);
    }

    let (version, digest) = signature
        .split_once('=')
        .ok_or(WebhookError::InvalidSignatureHeader)?;
    if version != SIGNATURE_VERSION || digest.is_empty() {
        return Err(WebhookError::InvalidSignatureHeader);
    }
    let signature = hex::decode(digest).map_err(|_| WebhookError::InvalidSignatureHeader)?;
    let mut mac =
        HmacSha256::new_from_slice(secret.as_ref()).expect("HMAC accepts keys of any size");
    mac.update(&signed_payload(timestamp, payload));
    mac.verify_slice(&signature)
        .map_err(|_| WebhookError::InvalidSignature)
}

fn signed_payload(timestamp: u64, payload: &[u8]) -> Vec<u8> {
    let mut signed = format!("{}:{}:", SIGNATURE_VERSION, timestamp).into_bytes();
    signed.extend_from_slice(payload);
    signed
}

fn parse_event_notification<'a>(
    client: &'a crate::Client,
    payload: &[u8],
) -> Result<EventNotification<'a>, WebhookError> {
    let event: RawEvent = serde_json::from_slice(payload).map_err(WebhookError::InvalidPayload)?;
    parse_known_event(client, event)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn signature_for(secret: &str, timestamp: u64, payload: &[u8]) -> String {
        let mut mac =
            HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC accepts keys of any size");
        mac.update(&signed_payload(timestamp, payload));
        format!(
            "{}={}",
            SIGNATURE_VERSION,
            hex::encode(mac.finalize().into_bytes())
        )
    }

    fn test_secret() -> String {
        ["whsec", "test"].join("_")
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_secs()
    }

    fn checkout_payload(base_url: &str, event_type: &str) -> Vec<u8> {
        format!(
            r#"{{
                "id":"evt_123",
                "type":"{event_type}",
                "created_at":"2026-04-10T12:00:00Z",
                "object":{{
                    "id":"chk_123",
                    "type":"checkout",
                    "url":"{base_url}/v0.1/checkouts/chk_123"
                }}
            }}"#
        )
        .into_bytes()
    }

    fn member_payload(base_url: &str, event_type: &str) -> Vec<u8> {
        format!(
            r#"{{
                "id":"evt_789",
                "type":"{event_type}",
                "created_at":"2026-04-10T12:00:00Z",
                "object":{{
                    "id":"mem_123",
                    "type":"member",
                    "url":"{base_url}/v0.1/merchants/MCODE/members/mem_123"
                }}
            }}"#
        )
        .into_bytes()
    }

    #[test]
    fn verifies_valid_signature() {
        let payload = checkout_payload("https://api.sumup.com", "checkout.paid");
        let timestamp = current_timestamp();
        let secret = test_secret();
        let signature = signature_for(&secret, timestamp, &payload);

        let result = verify_signature(&secret, &payload, signature, timestamp.to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_invalid_signature() {
        let payload = checkout_payload("https://api.sumup.com", "checkout.paid");
        let timestamp = current_timestamp();
        let secret = test_secret();

        let result = verify_signature(&secret, &payload, "v1=deadbeef", timestamp.to_string());

        assert!(matches!(result, Err(WebhookError::InvalidSignature)));
    }

    #[test]
    fn rejects_invalid_signature_header_format() {
        let payload = checkout_payload("https://api.sumup.com", "checkout.paid");
        let timestamp = current_timestamp();
        let secret = test_secret();

        let result = verify_signature(&secret, &payload, "not-hex", timestamp.to_string());

        assert!(matches!(result, Err(WebhookError::InvalidSignatureHeader)));
    }

    #[test]
    fn rejects_invalid_timestamp_header() {
        let payload = checkout_payload("https://api.sumup.com", "checkout.paid");
        let timestamp = current_timestamp();
        let secret = test_secret();
        let signature = signature_for(&secret, timestamp, &payload);

        let result = verify_signature(&secret, &payload, signature, "not-a-timestamp");

        assert!(matches!(
            result,
            Err(WebhookError::InvalidTimestampHeader(_))
        ));
    }

    #[test]
    fn rejects_expired_timestamp() {
        let payload = checkout_payload("https://api.sumup.com", "checkout.paid");
        let now = current_timestamp();
        let timestamp = now - DEFAULT_TOLERANCE.as_secs() - 1;
        let secret = test_secret();
        let signature = signature_for(&secret, timestamp, &payload);

        let result = WebhookHandler::new(&crate::Client::default(), &secret)
            .with_tolerance(Duration::from_secs(DEFAULT_TOLERANCE.as_secs()))
            .verify(&payload, signature, timestamp.to_string());

        assert!(matches!(result, Err(WebhookError::SignatureExpired)));
    }

    #[test]
    fn parses_checkout_events_into_typed_variants() {
        let client = crate::Client::default();
        let payload = checkout_payload("https://api.sumup.com", "checkout.paid");

        let event = client
            .parse_event_notification(&payload)
            .expect("parse checkout event");

        match event {
            EventNotification::CheckoutPaid(event) => {
                assert_eq!(event.id, "evt_123");
                assert_eq!(event.event_type(), "checkout.paid");
                assert_eq!(event.object.id, "chk_123");
            }
            other => panic!("expected checkout.paid event, got {:?}", other),
        }
    }

    #[test]
    fn parses_member_events_into_typed_variants() {
        let client = crate::Client::default();
        let payload = member_payload("https://api.sumup.com", "member.updated");

        let event = client
            .parse_event_notification(&payload)
            .expect("parse member event");

        match event {
            EventNotification::MemberUpdated(event) => {
                assert_eq!(event.id, "evt_789");
                assert_eq!(event.event_type(), "member.updated");
                assert_eq!(event.object.id, "mem_123");
            }
            other => panic!("expected member.updated event, got {:?}", other),
        }
    }

    #[test]
    fn rejects_mismatched_object_types_for_typed_events() {
        let client = crate::Client::default();
        let payload = br#"{
            "id":"evt_123",
            "type":"checkout.created",
            "created_at":"2026-04-10T12:00:00Z",
            "object":{
                "id":"mem_123",
                "type":"member",
                "url":"https://api.sumup.com/v0.1/merchants/MCODE/members/mem_123"
            }
        }"#;

        let result = client.parse_event_notification(payload);

        assert!(matches!(
            result,
            Err(WebhookError::UnexpectedObjectType {
                event_type,
                object_type
            }) if event_type == "checkout.created" && object_type == "member"
        ));
    }

    #[test]
    fn falls_back_to_unknown_for_other_events() {
        let client = crate::Client::default();
        let payload = br#"{
            "id":"evt_456",
            "type":"merchant.updated",
            "created_at":"2026-04-10T12:00:00Z",
            "object":{
                "id":"mrc_123",
                "type":"merchant",
                "url":"https://api.sumup.com/v0.1/me"
            }
        }"#;

        let event = client
            .parse_event_notification(payload)
            .expect("parse generic event");

        match event {
            EventNotification::Unknown(event) => {
                assert_eq!(event.event_type, "merchant.updated");
                assert_eq!(event.object.object_type, "merchant");
            }
            other => panic!("expected unknown event, got {:?}", other),
        }
    }

    #[test]
    fn verifies_and_parses_event_notifications() {
        let client = crate::Client::default();
        let payload = checkout_payload("https://api.sumup.com", "checkout.failed");
        let timestamp = current_timestamp();
        let secret = test_secret();
        let signature = signature_for(&secret, timestamp, &payload);

        let event = client
            .verify_and_parse_event_notification(
                &secret,
                &payload,
                signature,
                timestamp.to_string(),
            )
            .expect("verify and parse webhook");

        assert!(matches!(event, EventNotification::CheckoutFailed(_)));
    }

    #[test]
    fn client_creates_webhook_handler() {
        let client = crate::Client::default();
        let payload = checkout_payload("https://api.sumup.com", "checkout.failed");
        let timestamp = current_timestamp();
        let secret = test_secret();
        let signature = signature_for(&secret, timestamp, &payload);

        let event = client
            .webhook_handler(&secret)
            .verify_and_parse(&payload, signature, timestamp.to_string())
            .expect("verify and parse webhook");

        assert!(matches!(event, EventNotification::CheckoutFailed(_)));
    }

    #[test]
    fn marks_typed_events_as_fetchable() {
        let client = crate::Client::default();
        let created = client
            .parse_event_notification(&checkout_payload(
                "https://api.sumup.com",
                "checkout.created",
            ))
            .expect("parse checkout.created");
        let updated = client
            .parse_event_notification(&member_payload("https://api.sumup.com", "member.updated"))
            .expect("parse member.updated");
        let paid = client
            .parse_event_notification(&checkout_payload("https://api.sumup.com", "checkout.paid"))
            .expect("parse checkout.paid");
        let unknown = client
            .parse_event_notification(
                br#"{
                    "id":"evt_456",
                    "type":"merchant.updated",
                    "created_at":"2026-04-10T12:00:00Z",
                    "object":{
                        "id":"mrc_123",
                        "type":"merchant",
                        "url":"https://api.sumup.com/v0.1/me"
                    }
                }"#,
            )
            .expect("parse merchant.updated");

        assert!(created.should_fetch_object());
        assert!(updated.should_fetch_object());
        assert!(paid.should_fetch_object());
        assert!(!unknown.should_fetch_object());
    }

    #[tokio::test]
    async fn rejects_invalid_checkout_object_urls() {
        let client = crate::Client::default();
        let payload = br#"{
            "id":"evt_123",
            "type":"checkout.created",
            "created_at":"2026-04-10T12:00:00Z",
            "object":{
                "id":"chk_123",
                "type":"checkout",
                "url":"://not-a-valid-url"
            }
        }"#;

        let event = client
            .parse_event_notification(payload)
            .expect("parse checkout.created");

        match event {
            EventNotification::CheckoutCreated(event) => {
                let result = event.fetch_object().await;
                assert!(matches!(
                    result,
                    Err(WebhookFetchError::InvalidObjectUrl(_))
                ));
            }
            other => panic!("expected checkout.created event, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn fetches_checkout_objects_from_typed_events() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v0.1/checkouts/chk_123"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{
                    "id":"chk_123",
                    "checkout_reference":"order-123",
                    "amount":10.0,
                    "currency":"EUR",
                    "merchant_code":"MCODE",
                    "status":"PENDING"
                }"#,
                "application/json",
            ))
            .mount(&server)
            .await;

        let client = crate::Client::default().with_base_url(server.uri());
        let payload = checkout_payload(&server.uri(), "checkout.created");

        let event = client
            .parse_event_notification(&payload)
            .expect("parse checkout.created");

        match event {
            EventNotification::CheckoutCreated(event) => {
                let checkout = event.fetch_object().await.expect("fetch checkout");
                assert_eq!(checkout.id.as_deref(), Some("chk_123"));
                assert_eq!(checkout.status.as_deref(), Some("PENDING"));
            }
            other => panic!("expected checkout.created event, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn fetches_member_objects_from_typed_events() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v0.1/merchants/MCODE/members/mem_123"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{
                    "id":"mem_123",
                    "roles":["admin"],
                    "permissions":[],
                    "created_at":"2026-04-10T12:00:00Z",
                    "updated_at":"2026-04-10T12:00:00Z",
                    "status":"active"
                }"#,
                "application/json",
            ))
            .mount(&server)
            .await;

        let client = crate::Client::default().with_base_url(server.uri());
        let payload = member_payload(&server.uri(), "member.created");

        let event = client
            .parse_event_notification(&payload)
            .expect("parse member.created");

        match event {
            EventNotification::MemberCreated(event) => {
                let member = event.fetch_object().await.expect("fetch member");
                assert_eq!(member.id, "mem_123");
                assert_eq!(member.roles, vec!["admin"]);
            }
            other => panic!("expected member.created event, got {:?}", other),
        }
    }
}
