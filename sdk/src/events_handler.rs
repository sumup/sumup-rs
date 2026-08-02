//! Event notification verification and handling helpers.
//!
//! Most integrations should create an [`EventsHandler`] through
//! [`crate::Client::events_handler`], register typed callbacks with
//! the generated `on_*` methods, and pass the raw HTTP request body and SumUp signature
//! headers to [`EventsHandler::handle`].

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::{
    collections::BTreeMap,
    future::Future,
    pin::Pin,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

type HmacSha256 = Hmac<Sha256>;

/// HTTP header containing the event payload signature.
pub const SIGNATURE_HEADER: &str = "X-SumUp-Webhook-Signature";

/// HTTP header containing the Unix timestamp used for signature verification.
pub const TIMESTAMP_HEADER: &str = "X-SumUp-Webhook-Timestamp";

/// Event signature scheme version accepted by the SDK.
pub const SIGNATURE_VERSION: &str = "v1";

/// Default maximum allowed clock skew for event signature verification.
pub const DEFAULT_TOLERANCE: Duration = Duration::from_secs(5 * 60);

/// Error returned when an event cannot be verified or parsed.
#[derive(Debug)]
pub enum EventError {
    /// The signature header was missing or empty.
    MissingSignature,
    /// The timestamp header was missing or empty.
    MissingTimestamp,
    /// The signature header did not use the expected versioned hex format.
    InvalidSignatureHeader,
    /// The timestamp header was not a valid Unix timestamp.
    InvalidTimestampHeader(std::num::ParseIntError),
    /// The signature did not match the raw request body.
    InvalidSignature,
    /// The timestamp was outside the configured tolerance window.
    SignatureExpired,
    /// The request body was not valid JSON for the expected event shape.
    InvalidPayload(serde_json::Error),
}

impl std::fmt::Display for EventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSignature => write!(f, "missing event signature header"),
            Self::MissingTimestamp => write!(f, "missing event timestamp header"),
            Self::InvalidSignatureHeader => write!(f, "invalid event signature header"),
            Self::InvalidTimestampHeader(err) => {
                write!(f, "invalid event timestamp header: {}", err)
            }
            Self::InvalidSignature => write!(f, "invalid event signature"),
            Self::SignatureExpired => write!(f, "event timestamp outside allowed tolerance"),
            Self::InvalidPayload(err) => write!(f, "invalid event payload: {}", err),
        }
    }
}

impl std::error::Error for EventError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidTimestampHeader(err) => Some(err),
            Self::InvalidPayload(err) => Some(err),
            Self::MissingSignature
            | Self::MissingTimestamp
            | Self::InvalidSignatureHeader
            | Self::InvalidSignature
            | Self::SignatureExpired => None,
        }
    }
}

/// Boxed error returned by an event callback.
pub type EventCallbackError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Converts an async event callback's output into the result expected by the
/// events handler.
///
/// Callbacks can return either `()` or `Result<(), E>`, where `E` can be
/// converted into [`EventCallbackError`].
pub trait IntoEventHandlerResult {
    #[doc(hidden)]
    fn into_event_handler_result(self) -> Result<(), EventCallbackError>;
}

impl IntoEventHandlerResult for () {
    fn into_event_handler_result(self) -> Result<(), EventCallbackError> {
        Ok(())
    }
}

impl<E> IntoEventHandlerResult for Result<(), E>
where
    E: Into<EventCallbackError>,
{
    fn into_event_handler_result(self) -> Result<(), EventCallbackError> {
        self.map_err(Into::into)
    }
}

/// Error returned when an event callback cannot be registered.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EventHandlerRegistrationError {
    /// A callback was already registered for this event type.
    AlreadyRegistered(&'static str),
}

impl std::fmt::Display for EventHandlerRegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyRegistered(event_type) => {
                write!(f, "handler already registered for event type: {event_type}")
            }
        }
    }
}

impl std::error::Error for EventHandlerRegistrationError {}

/// Error returned when a notification cannot be verified, parsed, or handled.
#[derive(Debug)]
#[non_exhaustive]
pub enum EventHandlingError {
    /// The event could not be verified or parsed.
    Event(EventError),
    /// A registered or fallback callback failed.
    Callback(EventCallbackError),
}

impl From<EventError> for EventHandlingError {
    fn from(value: EventError) -> Self {
        Self::Event(value)
    }
}

impl std::fmt::Display for EventHandlingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Event(err) => write!(f, "failed to process event notification: {err}"),
            Self::Callback(err) => write!(f, "event callback failed: {err}"),
        }
    }
}

impl std::error::Error for EventHandlingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Event(err) => Some(err),
            Self::Callback(err) => Some(err.as_ref()),
        }
    }
}

type EventCallbackFuture =
    Pin<Box<dyn Future<Output = Result<(), EventCallbackError>> + Send + 'static>>;
type EventCallback =
    dyn Fn(crate::events::RawEvent, crate::Client) -> EventCallbackFuture + Send + Sync + 'static;

/// Verifies, parses, and routes event notifications to typed async callbacks.
///
/// Create this handler through [`crate::Client::events_handler`], then register
/// callbacks with the generated `on_*` methods before sharing it with your HTTP server.
/// Events without a dedicated callback are sent to the fallback callback
/// supplied at construction time.
///
/// ```no_run
/// # use sumup::{Client, events::{EventHandlerRegistrationError, EventNotification}};
/// # use sumup::members::MemberUpdatedEvent;
/// # async fn handle_unhandled(_event: EventNotification, _client: Client) {}
/// # async fn handle_member_updated(_event: MemberUpdatedEvent, _client: Client) {}
/// # fn configure() -> Result<(), EventHandlerRegistrationError> {
/// let client = Client::default();
/// let mut handler = client.events_handler("event_secret", handle_unhandled);
/// handler.on_member_updated(handle_member_updated)?;
/// # Ok(())
/// # }
/// ```
///
/// Each registration method requires the matching event type:
///
/// ```compile_fail,E0631
/// use sumup::{Client, readers::ReaderCreatedEvent};
///
/// async fn handle_reader_created(_event: ReaderCreatedEvent, _client: Client) {}
/// let mut handler = Client::default().events_handler("event_secret", |_, _| async {});
/// handler.on_member_updated(handle_reader_created).unwrap();
/// ```
pub struct EventsHandler {
    client: crate::Client,
    secret: Vec<u8>,
    tolerance: Duration,
    fallback: Box<EventCallback>,
    registered_handlers: BTreeMap<&'static str, Box<EventCallback>>,
}

impl std::fmt::Debug for EventsHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventsHandler")
            .field("tolerance", &self.tolerance)
            .field("registered_event_types", &self.registered_event_types())
            .finish_non_exhaustive()
    }
}

impl EventsHandler {
    /// Creates a handler with a callback for events that do not have a dedicated
    /// registered callback.
    pub fn new<Handler, HandlerFuture>(
        client: &crate::Client,
        secret: impl AsRef<[u8]>,
        fallback: Handler,
    ) -> Self
    where
        Handler: Fn(crate::events::EventNotification, crate::Client) -> HandlerFuture
            + Send
            + Sync
            + 'static,
        HandlerFuture: Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        let fallback = Box::new(move |event, client: crate::Client| {
            let event = crate::events::parse_known_event(client.clone(), event);
            let future = fallback(event, client);
            Box::pin(async move { future.await.into_event_handler_result() }) as EventCallbackFuture
        });

        Self {
            client: client.clone(),
            secret: secret.as_ref().to_vec(),
            tolerance: DEFAULT_TOLERANCE,
            fallback,
            registered_handlers: BTreeMap::new(),
        }
    }

    /// Returns the client used for parsing and follow-up API calls.
    pub fn client(&self) -> &crate::Client {
        &self.client
    }

    /// Overrides the allowed clock skew for event signature verification.
    ///
    /// The default tolerance is [`DEFAULT_TOLERANCE`]. Prefer a short tolerance
    /// in production so old signed requests cannot be replayed indefinitely.
    pub fn with_tolerance(mut self, tolerance: Duration) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub(crate) fn register<EventType, HandlerFuture>(
        &mut self,
        callback: impl Fn(crate::events::Event<EventType>, crate::Client) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        EventType: crate::events::EventSpec + Send + Sync + 'static,
        HandlerFuture: Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        let event_type = EventType::EVENT_TYPE;
        if self.registered_handlers.contains_key(event_type) {
            return Err(EventHandlerRegistrationError::AlreadyRegistered(event_type));
        }

        self.registered_handlers.insert(
            event_type,
            Box::new(move |event, client| {
                let event = crate::events::Event::<EventType>::from_raw(client.clone(), event);
                let future = callback(event, client);
                Box::pin(async move { future.await.into_event_handler_result() })
            }),
        );
        Ok(self)
    }

    /// Returns the event types with dedicated callbacks, sorted by event type.
    pub fn registered_event_types(&self) -> Vec<&'static str> {
        self.registered_handlers.keys().copied().collect()
    }

    /// Verifies, parses, and routes one raw event notification.
    ///
    /// Pass the exact raw body bytes received over HTTP. Do not parse,
    /// reserialize, trim, or otherwise transform the body before calling this
    /// method.
    pub async fn handle(
        &self,
        payload: &[u8],
        signature_header: impl AsRef<str>,
        timestamp_header: impl AsRef<str>,
    ) -> Result<(), EventHandlingError> {
        verify_signature_with_tolerance(
            &self.secret,
            payload,
            signature_header,
            timestamp_header,
            self.tolerance,
        )?;

        let event = parse_raw_event(payload)?;
        let callback = self
            .registered_handlers
            .get(event.event_type())
            .unwrap_or(&self.fallback);
        callback(event, self.client.clone())
            .await
            .map_err(EventHandlingError::Callback)
    }
}

pub(crate) fn parse_event_notification(
    client: &crate::Client,
    payload: &[u8],
) -> Result<crate::events::EventNotification, EventError> {
    let event = parse_raw_event(payload)?;
    Ok(crate::events::parse_known_event(client.clone(), event))
}

fn parse_raw_event(payload: &[u8]) -> Result<crate::events::RawEvent, EventError> {
    serde_json::from_slice(payload).map_err(EventError::InvalidPayload)
}

pub(crate) async fn fetch_object<T>(
    client: &crate::Client,
    object_url: &str,
) -> crate::error::SdkResult<T>
where
    T: serde::de::DeserializeOwned,
{
    let object_url = reqwest::Url::parse(object_url).map_err(|err| {
        crate::error::SdkError::InvalidRequest(format!("invalid event object URL: {err}"))
    })?;
    let base_url = reqwest::Url::parse(client.base_url()).map_err(|err| {
        crate::error::SdkError::InvalidRequest(format!("invalid client base URL: {err}"))
    })?;
    if object_url.host_str() != base_url.host_str() {
        return Err(crate::error::SdkError::InvalidRequest(
            "event object URL host does not match client base URL host".into(),
        ));
    }

    let mut request = client
        .http_client()
        .get(object_url)
        .header("User-Agent", crate::version::user_agent())
        .timeout(client.timeout());
    if let Some(authorization) = client.authorization() {
        request = request.header("Authorization", format!("Bearer {}", authorization));
    }
    for (header_name, header_value) in client.runtime_headers() {
        request = request.header(*header_name, header_value);
    }

    let response = request.send().await?;
    let status = response.status();
    match status {
        reqwest::StatusCode::OK => response.json().await.map_err(Into::into),
        _ => {
            let body_bytes = response.bytes().await?;
            let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
            Err(crate::error::SdkError::unexpected(status, body))
        }
    }
}

impl crate::Client {
    /// Verifies and parses an event notification using this client.
    ///
    /// Pass the raw HTTP request body and the `X-SumUp-Webhook-Signature` and
    /// `X-SumUp-Webhook-Timestamp` header values from the same request.
    pub fn parse_event_notification(
        &self,
        secret: impl AsRef<[u8]>,
        payload: &[u8],
        signature_header: impl AsRef<str>,
        timestamp_header: impl AsRef<str>,
    ) -> Result<crate::events::EventNotification, EventError> {
        verify_signature(secret, payload, signature_header, timestamp_header)?;
        parse_event_notification(self, payload)
    }

    /// Parses an event notification without verifying its signature.
    ///
    /// Only use this for tests, fixtures, or payloads that were already verified
    /// by trusted infrastructure before entering this process. Prefer
    /// [`crate::Client::parse_event_notification`] for production request handling.
    pub fn dangerously_parse_unverified_event_notification(
        &self,
        payload: &[u8],
    ) -> Result<crate::events::EventNotification, EventError> {
        parse_event_notification(self, payload)
    }
}

/// Verifies that event signature headers match the raw request body.
///
/// This is useful when your integration wants to verify the request before
/// handing the body to another component. If you want a typed SDK event, prefer
/// [`crate::Client::parse_event_notification`].
pub fn verify_signature(
    secret: impl AsRef<[u8]>,
    payload: &[u8],
    signature_header: impl AsRef<str>,
    timestamp_header: impl AsRef<str>,
) -> Result<(), EventError> {
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
) -> Result<(), EventError> {
    let signature = signature_header.as_ref().trim();
    if signature.is_empty() {
        return Err(EventError::MissingSignature);
    }

    let timestamp = timestamp_header.as_ref().trim();
    if timestamp.is_empty() {
        return Err(EventError::MissingTimestamp);
    }
    let timestamp = timestamp
        .parse::<u64>()
        .map_err(EventError::InvalidTimestampHeader)?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_secs();
    if now.abs_diff(timestamp) > tolerance.as_secs() {
        return Err(EventError::SignatureExpired);
    }

    let (version, digest) = signature
        .split_once('=')
        .ok_or(EventError::InvalidSignatureHeader)?;
    if version != SIGNATURE_VERSION || digest.is_empty() {
        return Err(EventError::InvalidSignatureHeader);
    }
    let signature = hex::decode(digest).map_err(|_| EventError::InvalidSignatureHeader)?;
    let mut mac =
        HmacSha256::new_from_slice(secret.as_ref()).expect("HMAC accepts keys of any size");
    mac.update(format!("{SIGNATURE_VERSION}:{timestamp}:").as_bytes());
    mac.update(payload);
    mac.verify_slice(&signature)
        .map_err(|_| EventError::InvalidSignature)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{EventNotification, FetchObject};
    use std::sync::atomic::Ordering;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path, query_param},
    };

    fn signature_for(secret: &str, timestamp: u64, payload: &[u8]) -> String {
        let mut mac =
            HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC accepts keys of any size");
        let mut signed = format!("v1:{timestamp}:").into_bytes();
        signed.extend_from_slice(payload);
        mac.update(&signed);
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

    fn reader_payload(base_url: &str, event_type: &str) -> Vec<u8> {
        format!(
            r#"{{
                "id":"evt_123",
                "type":"{event_type}",
                "created_at":"2026-04-10T12:00:00Z",
                "object":{{
                    "id":"rdr_123",
                    "type":"reader",
                    "url":"{base_url}/v0.1/merchants/MCODE/readers/rdr_123"
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
    fn verifies_fixed_signature_fixture() {
        // Generated independently with OpenSSL over v1:1234567890:{"id":"evt_123"}.
        let signature = "v1=02e9076b318aadab2e3d14549950465512b32a100ea122b5bcb815f13d4b3153";
        for (payload, valid) in [
            (br#"{"id":"evt_123"}"#.as_slice(), true),
            (br#"{"id":"evt_124"}"#.as_slice(), false),
        ] {
            let result = verify_signature_with_tolerance(
                "test-secret",
                payload,
                signature,
                "1234567890",
                Duration::MAX,
            );
            if valid {
                result.expect("verify independently signed fixture");
            } else {
                assert!(matches!(result, Err(EventError::InvalidSignature)));
            }
        }
    }

    #[test]
    fn verifies_valid_signature() {
        let payload = member_payload("https://api.sumup.com", "members.updated");
        let timestamp = current_timestamp();
        let secret = test_secret();
        let signature = signature_for(&secret, timestamp, &payload);

        let result = verify_signature(&secret, &payload, signature, timestamp.to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_invalid_signature() {
        let payload = member_payload("https://api.sumup.com", "members.updated");
        let timestamp = current_timestamp();
        let secret = test_secret();

        let result = verify_signature(&secret, &payload, "v1=deadbeef", timestamp.to_string());

        assert!(matches!(result, Err(EventError::InvalidSignature)));
    }

    #[test]
    fn rejects_invalid_signature_header_format() {
        let payload = member_payload("https://api.sumup.com", "members.updated");
        let timestamp = current_timestamp();
        let secret = test_secret();

        let result = verify_signature(&secret, &payload, "not-hex", timestamp.to_string());

        assert!(matches!(result, Err(EventError::InvalidSignatureHeader)));
    }

    #[test]
    fn rejects_invalid_timestamp_header() {
        let payload = member_payload("https://api.sumup.com", "members.updated");
        let timestamp = current_timestamp();
        let secret = test_secret();
        let signature = signature_for(&secret, timestamp, &payload);

        let result = verify_signature(&secret, &payload, signature, "not-a-timestamp");

        assert!(matches!(result, Err(EventError::InvalidTimestampHeader(_))));
    }

    #[tokio::test]
    async fn events_handler_honors_custom_tolerance() {
        let payload = member_payload("https://api.sumup.com", "members.updated");
        let now = current_timestamp();
        let tolerance = Duration::from_secs(1);
        let timestamp = now - tolerance.as_secs() - 1;
        let secret = test_secret();
        let signature = signature_for(&secret, timestamp, &payload);

        let handler = crate::Client::default()
            .events_handler(&secret, |_, _| async {})
            .with_tolerance(tolerance);
        let result = handler
            .handle(&payload, signature, timestamp.to_string())
            .await;

        assert!(matches!(
            result,
            Err(EventHandlingError::Event(EventError::SignatureExpired))
        ));
    }

    #[test]
    fn verifies_and_parses_event_notifications() {
        let client = crate::Client::default();
        let payload = member_payload("https://api.sumup.com", "members.updated");
        let timestamp = current_timestamp();
        let secret = test_secret();
        let signature = signature_for(&secret, timestamp, &payload);

        let event = client
            .parse_event_notification(&secret, &payload, signature, timestamp.to_string())
            .expect("verify and parse event");

        assert!(matches!(
            event,
            crate::events::EventNotification::MemberUpdated(_)
        ));
    }

    #[tokio::test]
    async fn events_handler_routes_typed_and_unhandled_events() {
        let client = crate::Client::default();
        let secret = test_secret();
        let handled = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let unhandled = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

        let fallback_calls = unhandled.clone();
        let mut handler = client.events_handler(&secret, move |event, _client| {
            let fallback_calls = fallback_calls.clone();
            async move {
                let is_unknown = matches!(&event, EventNotification::Unknown(_));
                fallback_calls
                    .lock()
                    .expect("lock fallback calls")
                    .push((event.event_type().to_owned(), is_unknown));
            }
        });

        let handled_calls = handled.clone();
        handler
            .on_member_updated(move |event, client| {
                let handled_calls = handled_calls.clone();
                async move {
                    handled_calls
                        .lock()
                        .expect("lock handled calls")
                        .push((event.event_type().to_owned(), client.base_url().to_owned()));
                }
            })
            .expect("register members.updated callback");

        let handled_calls = handled.clone();
        handler
            .on_reader_created(move |event, client| {
                let handled_calls = handled_calls.clone();
                async move {
                    handled_calls
                        .lock()
                        .expect("lock handled calls")
                        .push((event.event_type().to_owned(), client.base_url().to_owned()));
                }
            })
            .expect("register readers.created callback");

        let timestamp = current_timestamp();
        let typed_payload = member_payload("https://api.sumup.com", "members.updated");
        let typed_signature = signature_for(&secret, timestamp, &typed_payload);
        handler
            .handle(&typed_payload, typed_signature, timestamp.to_string())
            .await
            .expect("handle registered event");

        let reader_payload = reader_payload("https://api.sumup.com", "readers.created");
        let reader_signature = signature_for(&secret, timestamp, &reader_payload);
        handler
            .handle(&reader_payload, reader_signature, timestamp.to_string())
            .await
            .expect("handle registered reader event");

        let known_payload = member_payload("https://api.sumup.com", "members.created");
        let known_signature = signature_for(&secret, timestamp, &known_payload);
        handler
            .handle(&known_payload, known_signature, timestamp.to_string())
            .await
            .expect("handle known unregistered event");

        let unknown_payload = member_payload("https://api.sumup.com", "merchant.updated");
        let unknown_signature = signature_for(&secret, timestamp, &unknown_payload);
        handler
            .handle(&unknown_payload, unknown_signature, timestamp.to_string())
            .await
            .expect("handle unknown event");

        assert_eq!(
            *handled.lock().expect("lock handled calls"),
            vec![
                (
                    "members.updated".to_owned(),
                    "https://api.sumup.com".to_owned()
                ),
                (
                    "readers.created".to_owned(),
                    "https://api.sumup.com".to_owned()
                ),
            ]
        );
        assert_eq!(
            *unhandled.lock().expect("lock fallback calls"),
            vec![
                ("members.created".to_owned(), false),
                ("merchant.updated".to_owned(), true)
            ]
        );
        assert_eq!(
            handler.registered_event_types(),
            vec!["members.updated", "readers.created"]
        );
    }

    #[tokio::test]
    async fn events_handler_verifies_before_dispatching() {
        let client = crate::Client::default();
        let secret = test_secret();
        let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let fallback_calls = calls.clone();
        let mut handler = client.events_handler(&secret, move |_, _| {
            let fallback_calls = fallback_calls.clone();
            async move {
                fallback_calls.fetch_add(1, Ordering::Relaxed);
            }
        });
        let handled_calls = calls.clone();
        handler
            .on_member_updated(move |_, _| {
                let handled_calls = handled_calls.clone();
                async move {
                    handled_calls.fetch_add(1, Ordering::Relaxed);
                }
            })
            .expect("register callback");

        let payload = member_payload("https://api.sumup.com", "members.updated");
        let error = handler
            .handle(&payload, "v1=deadbeef", current_timestamp().to_string())
            .await
            .expect_err("reject invalid signature");

        assert!(matches!(
            error,
            EventHandlingError::Event(EventError::InvalidSignature)
        ));
        assert_eq!(calls.load(Ordering::Relaxed), 0);
    }

    #[tokio::test]
    async fn events_handler_rejects_duplicates_but_allows_registration_after_handling() {
        let client = crate::Client::default();
        let secret = test_secret();
        let mut handler = client.events_handler(&secret, |_, _| async {});

        handler
            .on_member_updated(|_, _| async {})
            .expect("register callback");
        let duplicate = handler
            .on_member_updated(|_, _| async {})
            .expect_err("reject duplicate callback");
        assert_eq!(
            duplicate,
            EventHandlerRegistrationError::AlreadyRegistered("members.updated")
        );

        let payload = member_payload("https://api.sumup.com", "members.updated");
        let timestamp = current_timestamp();
        let signature = signature_for(&secret, timestamp, &payload);
        handler
            .handle(&payload, signature, timestamp.to_string())
            .await
            .expect("handle event");

        handler
            .on_reader_created(|_, _| async {
                Err::<(), _>(std::io::Error::other("reader callback invoked"))
            })
            .expect("register callback after handling");

        let payload = reader_payload("https://api.sumup.com", "readers.created");
        let signature = signature_for(&secret, timestamp, &payload);
        let error = handler
            .handle(&payload, signature, timestamp.to_string())
            .await
            .expect_err("new callback is invoked");
        assert!(matches!(error, EventHandlingError::Callback(_)));
        assert_eq!(
            error.to_string(),
            "event callback failed: reader callback invoked"
        );
    }

    #[tokio::test]
    async fn events_handler_surfaces_fallback_errors() {
        let client = crate::Client::default();
        let secret = test_secret();
        let handler = client.events_handler(&secret, |_, _| async {
            Err::<(), _>(std::io::Error::other("fallback failed"))
        });

        let payload = member_payload("https://api.sumup.com", "members.updated");
        let timestamp = current_timestamp();
        let signature = signature_for(&secret, timestamp, &payload);
        let error = handler
            .handle(&payload, signature, timestamp.to_string())
            .await
            .expect_err("surface fallback error");

        assert!(matches!(error, EventHandlingError::Callback(_)));
        assert_eq!(error.to_string(), "event callback failed: fallback failed");
    }

    #[test]
    fn registered_event_types_are_sorted() {
        let mut handler = crate::Client::default().events_handler(test_secret(), |_, _| async {});
        handler.on_reader_created(|_, _| async {}).unwrap();
        handler.on_member_updated(|_, _| async {}).unwrap();

        assert_eq!(
            handler.registered_event_types(),
            vec!["members.updated", "readers.created"]
        );
    }

    #[tokio::test]
    async fn events_handler_surfaces_callback_errors() {
        let client = crate::Client::default();
        let secret = test_secret();
        let mut handler = client.events_handler(&secret, |_, _| async {});
        handler
            .on_member_updated(|_, _| async {
                Err::<(), _>(std::io::Error::other("callback failed"))
            })
            .expect("register callback");

        let payload = member_payload("https://api.sumup.com", "members.updated");
        let timestamp = current_timestamp();
        let signature = signature_for(&secret, timestamp, &payload);
        let error = handler
            .handle(&payload, signature, timestamp.to_string())
            .await
            .expect_err("surface callback error");

        assert!(matches!(error, EventHandlingError::Callback(_)));
        assert_eq!(error.to_string(), "event callback failed: callback failed");
    }

    #[test]
    fn parses_reader_events_into_typed_variants() {
        let client = crate::Client::default();
        let payload = reader_payload("https://api.sumup.com", "readers.created");

        let event = client
            .dangerously_parse_unverified_event_notification(&payload)
            .expect("parse reader event");

        match event {
            EventNotification::ReaderCreated(event) => {
                assert_eq!(event.id, "evt_123");
                assert_eq!(event.event_type(), "readers.created");
                assert_eq!(event.object.id, "rdr_123");
            }
            other => panic!("expected readers.created event, got {:?}", other),
        }
    }

    #[test]
    fn parses_member_events_into_typed_variants() {
        let client = crate::Client::default();
        let payload = member_payload("https://api.sumup.com", "members.updated");

        let event = client
            .dangerously_parse_unverified_event_notification(&payload)
            .expect("parse member event");

        match event {
            EventNotification::MemberUpdated(event) => {
                assert_eq!(event.id, "evt_789");
                assert_eq!(event.event_type(), "members.updated");
                assert_eq!(event.object.id, "mem_123");
            }
            other => panic!("expected members.updated event, got {:?}", other),
        }
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
            .dangerously_parse_unverified_event_notification(payload)
            .expect("parse generic event");

        match event {
            EventNotification::Unknown(event) => {
                assert_eq!(event.event_type, "merchant.updated");
                assert_eq!(event.object.object_type, "merchant");
            }
            other => panic!("expected unknown event, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn rejects_invalid_reader_object_urls() {
        let client = crate::Client::default();
        let payload = br#"{
            "id":"evt_123",
            "type":"readers.created",
            "created_at":"2026-04-10T12:00:00Z",
            "object":{
                "id":"rdr_123",
                "type":"reader",
                "url":"://not-a-valid-url"
            }
        }"#;

        let event = client
            .dangerously_parse_unverified_event_notification(payload)
            .expect("parse readers.created");

        match event {
            EventNotification::ReaderCreated(event) => {
                let result = event.fetch_object().await;
                assert!(matches!(
                    result,
                    Err(crate::error::SdkError::InvalidRequest(_))
                ));
            }
            other => panic!("expected readers.created event, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn rejects_object_urls_outside_client_base_url() {
        let client = crate::Client::default();
        let payload = reader_payload("https://api.sumup.com.evil", "readers.created");

        let event = client
            .dangerously_parse_unverified_event_notification(&payload)
            .expect("parse readers.created");

        match event {
            EventNotification::ReaderCreated(event) => {
                let result = event.fetch_object().await;
                assert!(matches!(
                    result,
                    Err(crate::error::SdkError::InvalidRequest(_))
                ));
            }
            other => panic!("expected readers.created event, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn rejects_invalid_client_base_url() {
        let client = crate::Client::default().with_base_url("://invalid");
        let error = fetch_object::<serde_json::Value>(&client, "https://api.sumup.com/object")
            .await
            .expect_err("reject invalid client base URL");

        assert!(matches!(error, crate::error::SdkError::InvalidRequest(_)));
        assert!(error.to_string().contains("invalid client base URL"));
    }

    #[tokio::test]
    async fn fetch_object_preserves_api_errors() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/object"))
            .respond_with(ResponseTemplate::new(404).set_body_string("object not found"))
            .expect(1)
            .mount(&server)
            .await;
        let client = crate::Client::default().with_base_url(server.uri());
        let error = fetch_object::<serde_json::Value>(&client, &format!("{}/object", server.uri()))
            .await
            .expect_err("return SDK error");

        assert!(matches!(error, crate::error::SdkError::Unexpected(_, _)));
        assert_eq!(error.status(), Some(reqwest::StatusCode::NOT_FOUND));
        assert_eq!(
            error.unexpected_body(),
            Some(&crate::error::UnknownApiBody::Text(
                "object not found".into()
            ))
        );
    }

    #[tokio::test]
    async fn fetches_reader_objects_from_typed_events() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v0.1/merchants/MCODE/readers/rdr_123"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{
                    "id":"rdr_123",
                    "name":"Front desk",
                    "status":"paired",
                    "created_at":"2026-04-10T12:00:00Z",
                    "updated_at":"2026-04-10T12:00:00Z",
                    "device":{
                        "identifier":"U1DT3NA00-CN",
                        "model":"solo"
                    }
                }"#,
                "application/json",
            ))
            .mount(&server)
            .await;

        let client = crate::Client::default().with_base_url(server.uri());
        let payload = reader_payload(&server.uri(), "readers.created");

        let event = client
            .dangerously_parse_unverified_event_notification(&payload)
            .expect("parse readers.created");

        match event {
            EventNotification::ReaderCreated(event) => {
                let reader = event.fetch_object().await.expect("fetch reader");
                assert_eq!(reader.id, "rdr_123");
                assert_eq!(
                    reader.status,
                    crate::resources::readers::ReaderStatus::Paired
                );
            }
            other => panic!("expected readers.created event, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn fetches_object_with_path_and_query_only_after_host_validation() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v0.1/merchants/MCODE/readers/rdr_123"))
            .and(query_param("expand", "payments"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{
                    "id":"rdr_123",
                    "name":"Front desk",
                    "status":"paired",
                    "created_at":"2026-04-10T12:00:00Z",
                    "updated_at":"2026-04-10T12:00:00Z",
                    "device":{
                        "identifier":"U1DT3NA00-CN",
                        "model":"solo"
                    }
                }"#,
                "application/json",
            ))
            .mount(&server)
            .await;

        let client = crate::Client::default().with_base_url(server.uri());
        let payload = format!(
            r#"{{
                "id":"evt_123",
                "type":"readers.created",
                "created_at":"2026-04-10T12:00:00Z",
                "object":{{
                    "id":"rdr_123",
                    "type":"reader",
                    "url":"{}/v0.1/merchants/MCODE/readers/rdr_123?expand=payments"
                }}
            }}"#,
            server.uri()
        )
        .into_bytes();

        let event = client
            .dangerously_parse_unverified_event_notification(&payload)
            .expect("parse readers.created");

        match event {
            EventNotification::ReaderCreated(event) => {
                let reader = event.fetch_object().await.expect("fetch reader");
                assert_eq!(reader.id, "rdr_123");
            }
            other => panic!("expected readers.created event, got {:?}", other),
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
        let payload = member_payload(&server.uri(), "members.created");

        let event = client
            .dangerously_parse_unverified_event_notification(&payload)
            .expect("parse members.created");

        match event {
            EventNotification::MemberCreated(event) => {
                let member = event.fetch_object().await.expect("fetch member");
                assert_eq!(member.id, "mem_123");
                assert_eq!(member.roles, vec!["admin"]);
            }
            other => panic!("expected members.created event, got {:?}", other),
        }
    }
}
