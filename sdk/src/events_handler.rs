//! Event notification verification helpers.
//!
//! Most integrations should create an [`EventNotificationHandler`] through
//! [`crate::Client::event_notification_handler`], register typed callbacks with
//! [`EventNotificationHandler::on`], and pass the raw HTTP request body and
//! SumUp signature headers to [`EventNotificationHandler::handle`].

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
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
/// notification handler.
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
    /// The handler has already handled an event and can no longer be changed.
    RegistrationClosed,
}

impl std::fmt::Display for EventHandlerRegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyRegistered(event_type) => {
                write!(f, "handler already registered for event type: {event_type}")
            }
            Self::RegistrationClosed => {
                write!(
                    f,
                    "cannot register event callbacks after handling has started"
                )
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

/// Error returned when fetching the API resource referenced by an event.
#[derive(Debug)]
#[non_exhaustive]
pub enum EventFetchError {
    /// The referenced object URL was not a valid absolute URL.
    InvalidObjectUrl(String),
    /// The API request failed or returned a non-success response.
    Sdk(crate::error::SdkError),
}

impl From<crate::error::SdkError> for EventFetchError {
    fn from(value: crate::error::SdkError) -> Self {
        Self::Sdk(value)
    }
}

impl std::fmt::Display for EventFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidObjectUrl(url) => write!(f, "invalid event object url: {}", url),
            Self::Sdk(err) => write!(f, "failed to fetch event object: {}", err),
        }
    }
}

impl std::error::Error for EventFetchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Sdk(err) => Some(err),
            Self::InvalidObjectUrl(_) => None,
        }
    }
}

/// Verifies and parses event notifications for a [`crate::Client`].
#[derive(Debug, Clone)]
pub struct EventsHandler {
    client: crate::Client,
    secret: Vec<u8>,
    tolerance: Duration,
}

impl EventsHandler {
    /// Creates an event handler using the signing secret configured for your
    /// SumUp event destination.
    pub fn new(client: &crate::Client, secret: impl AsRef<[u8]>) -> Self {
        Self {
            client: client.clone(),
            secret: secret.as_ref().to_vec(),
            tolerance: DEFAULT_TOLERANCE,
        }
    }

    /// Returns the client used for parsing and follow-up resource fetches.
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

    /// Verifies that the headers match the raw request body.
    ///
    /// Use this when you need separate verification and parsing steps. Most
    /// integrations should call [`EventsHandler::parse`] instead.
    pub fn verify(
        &self,
        payload: &[u8],
        signature_header: impl AsRef<str>,
        timestamp_header: impl AsRef<str>,
    ) -> Result<(), EventError> {
        verify_signature_with_tolerance(
            &self.secret,
            payload,
            signature_header,
            timestamp_header,
            self.tolerance,
        )
    }

    /// Verifies the request and parses it into a typed event notification.
    ///
    /// Pass the exact raw body bytes received over HTTP. Do not parse,
    /// reserialize, trim, or otherwise transform the body before verification.
    pub fn parse(
        &self,
        payload: &[u8],
        signature_header: impl AsRef<str>,
        timestamp_header: impl AsRef<str>,
    ) -> Result<crate::events::EventNotification, EventError> {
        self.verify(payload, signature_header, timestamp_header)?;
        parse_event_notification(&self.client, payload)
    }

    /// Parses an event notification without verifying its signature.
    ///
    /// Only use this for tests, fixtures, or payloads that were already verified
    /// by trusted infrastructure before entering this process. Prefer
    /// [`EventsHandler::parse`] for production request handling.
    pub fn dangerously_parse_unverified(
        &self,
        payload: &[u8],
    ) -> Result<crate::events::EventNotification, EventError> {
        parse_event_notification(&self.client, payload)
    }
}

type EventCallbackFuture =
    Pin<Box<dyn Future<Output = Result<(), EventCallbackError>> + Send + 'static>>;
type RegisteredEventCallback =
    dyn Fn(crate::events::RawEvent, crate::Client) -> EventCallbackFuture + Send + Sync + 'static;
type FallbackEventCallback = dyn Fn(crate::events::EventNotification, crate::Client) -> EventCallbackFuture
    + Send
    + Sync
    + 'static;

/// Verifies, parses, and routes event notifications to typed async callbacks.
///
/// Create this handler through [`crate::Client::event_notification_handler`], then
/// register callbacks with [`EventNotificationHandler::on`] before sharing it
/// with your HTTP server. Events without a dedicated callback are sent to the
/// fallback callback supplied at construction time.
///
/// ```no_run
/// # use sumup::{Client, events::{EventHandlerRegistrationError, EventNotification}};
/// # use sumup::members::MemberUpdatedEvent;
/// # async fn handle_unhandled(_event: EventNotification, _client: Client) {}
/// # async fn handle_member_updated(_event: MemberUpdatedEvent, _client: Client) {}
/// # fn configure() -> Result<(), EventHandlerRegistrationError> {
/// let client = Client::default();
/// let mut handler = client.event_notification_handler("event_secret", handle_unhandled);
/// handler.on(handle_member_updated)?;
/// # Ok(())
/// # }
/// ```
pub struct EventNotificationHandler {
    events: EventsHandler,
    fallback: Box<FallbackEventCallback>,
    registered_handlers: HashMap<&'static str, Box<RegisteredEventCallback>>,
    has_handled_event: AtomicBool,
}

impl std::fmt::Debug for EventNotificationHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventNotificationHandler")
            .field("events", &self.events)
            .field("registered_event_types", &self.registered_event_types())
            .field(
                "has_handled_event",
                &self.has_handled_event.load(Ordering::Acquire),
            )
            .finish_non_exhaustive()
    }
}

impl EventNotificationHandler {
    /// Creates a handler with a callback for events that do not have a dedicated
    /// registered callback.
    pub fn new<Handler, HandlerFuture, HandlerOutput>(
        client: &crate::Client,
        secret: impl AsRef<[u8]>,
        fallback: Handler,
    ) -> Self
    where
        Handler: Fn(crate::events::EventNotification, crate::Client) -> HandlerFuture
            + Send
            + Sync
            + 'static,
        HandlerFuture: Future<Output = HandlerOutput> + Send + 'static,
        HandlerOutput: IntoEventHandlerResult + 'static,
    {
        let fallback = Box::new(move |event, client| {
            let future = fallback(event, client);
            Box::pin(async move { future.await.into_event_handler_result() }) as EventCallbackFuture
        });

        Self {
            events: EventsHandler::new(client, secret),
            fallback,
            registered_handlers: HashMap::new(),
            has_handled_event: AtomicBool::new(false),
        }
    }

    /// Returns the client used for parsing and follow-up API calls.
    pub fn client(&self) -> &crate::Client {
        self.events.client()
    }

    /// Overrides the allowed clock skew for event signature verification.
    pub fn with_tolerance(mut self, tolerance: Duration) -> Self {
        self.events = self.events.with_tolerance(tolerance);
        self
    }

    /// Registers an async callback for the generated event type accepted by the
    /// callback's first argument.
    ///
    /// The callback also receives a clone of the client associated with this
    /// handler. Only one callback can be registered for each event type, and all
    /// callbacks must be registered before the first call to
    /// [`EventNotificationHandler::handle`].
    pub fn on<EventType, Handler, HandlerFuture, HandlerOutput>(
        &mut self,
        callback: Handler,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        EventType: crate::events::EventSpec + Send + Sync + 'static,
        Handler: Fn(crate::events::Event<EventType>, crate::Client) -> HandlerFuture
            + Send
            + Sync
            + 'static,
        HandlerFuture: Future<Output = HandlerOutput> + Send + 'static,
        HandlerOutput: IntoEventHandlerResult + 'static,
    {
        if self.has_handled_event.load(Ordering::Acquire) {
            return Err(EventHandlerRegistrationError::RegistrationClosed);
        }

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
        let mut event_types: Vec<_> = self.registered_handlers.keys().copied().collect();
        event_types.sort_unstable();
        event_types
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
        self.has_handled_event.store(true, Ordering::Release);
        self.events
            .verify(payload, signature_header, timestamp_header)?;

        let event = parse_raw_event(payload)?;
        let client = self.events.client().clone();
        if let Some(callback) = self.registered_handlers.get(event.event_type()) {
            return callback(event, client)
                .await
                .map_err(EventHandlingError::Callback);
        }

        let event = crate::events::parse_known_event(client.clone(), event)?;
        (self.fallback)(event, client)
            .await
            .map_err(EventHandlingError::Callback)
    }
}

pub(crate) fn parse_event_notification(
    client: &crate::Client,
    payload: &[u8],
) -> Result<crate::events::EventNotification, EventError> {
    let event = parse_raw_event(payload)?;
    crate::events::parse_known_event(client.clone(), event)
}

fn parse_raw_event(payload: &[u8]) -> Result<crate::events::RawEvent, EventError> {
    serde_json::from_slice(payload).map_err(EventError::InvalidPayload)
}

pub(crate) async fn fetch_object<T>(
    client: &crate::Client,
    object_url: &str,
) -> Result<T, EventFetchError>
where
    T: serde::de::DeserializeOwned,
{
    let object_url = reqwest::Url::parse(object_url)
        .map_err(|_| EventFetchError::InvalidObjectUrl(object_url.to_owned()))?;
    let base_url = reqwest::Url::parse(client.base_url())
        .map_err(|_| EventFetchError::InvalidObjectUrl(client.base_url().to_owned()))?;
    if !url_has_host(&object_url, &base_url) {
        return Err(EventFetchError::InvalidObjectUrl(object_url.to_string()));
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

    let response = request.send().await.map_err(crate::error::SdkError::from)?;
    let status = response.status();
    let result = match status {
        reqwest::StatusCode::OK => response.json().await.map_err(Into::into),
        _ => {
            let body_bytes = response
                .bytes()
                .await
                .map_err(crate::error::SdkError::from)?;
            let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
            Err(crate::error::SdkError::unexpected(status, body))
        }
    };

    result.map_err(Into::into)
}

fn url_has_host(url: &reqwest::Url, base_url: &reqwest::Url) -> bool {
    url.host_str() == base_url.host_str()
}

impl crate::Client {
    /// Verifies and parses an event notification using this client.
    ///
    /// This is a convenience wrapper around [`crate::Client::events_handler`] and
    /// [`EventsHandler::parse`]. Pass the raw HTTP request body and the
    /// `X-SumUp-Webhook-Signature` and `X-SumUp-Webhook-Timestamp` header
    /// values from the same request.
    pub fn parse_event_notification(
        &self,
        secret: impl AsRef<[u8]>,
        payload: &[u8],
        signature_header: impl AsRef<str>,
        timestamp_header: impl AsRef<str>,
    ) -> Result<crate::events::EventNotification, EventError> {
        self.events_handler(secret)
            .parse(payload, signature_header, timestamp_header)
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
/// [`EventsHandler::parse`] or [`crate::Client::parse_event_notification`].
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
    mac.update(&signed_payload(timestamp, payload));
    mac.verify_slice(&signature)
        .map_err(|_| EventError::InvalidSignature)
}

pub(crate) fn signed_payload(timestamp: u64, payload: &[u8]) -> Vec<u8> {
    let mut signed = format!("{}:{}:", SIGNATURE_VERSION, timestamp).into_bytes();
    signed.extend_from_slice(payload);
    signed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{EventNotification, FetchObject};
    use wiremock::{
        matchers::{method, path, query_param},
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

    #[test]
    fn rejects_expired_timestamp() {
        let payload = member_payload("https://api.sumup.com", "members.updated");
        let now = current_timestamp();
        let timestamp = now - DEFAULT_TOLERANCE.as_secs() - 1;
        let secret = test_secret();
        let signature = signature_for(&secret, timestamp, &payload);

        let result = EventsHandler::new(&crate::Client::default(), &secret)
            .with_tolerance(Duration::from_secs(DEFAULT_TOLERANCE.as_secs()))
            .verify(&payload, signature, timestamp.to_string());

        assert!(matches!(result, Err(EventError::SignatureExpired)));
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

    #[test]
    fn client_creates_events_handler() {
        let client = crate::Client::default();
        let payload = member_payload("https://api.sumup.com", "members.updated");
        let timestamp = current_timestamp();
        let secret = test_secret();
        let signature = signature_for(&secret, timestamp, &payload);

        let event = client
            .events_handler(&secret)
            .parse(&payload, signature, timestamp.to_string())
            .expect("verify and parse event");

        assert!(matches!(
            event,
            crate::events::EventNotification::MemberUpdated(_)
        ));
    }

    #[tokio::test]
    async fn event_notification_handler_routes_typed_and_unhandled_events() {
        let client = crate::Client::default();
        let secret = test_secret();
        let handled = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let unhandled = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

        let fallback_calls = unhandled.clone();
        let mut handler = client.event_notification_handler(&secret, move |event, _client| {
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
            .on(
                move |event: crate::resources::members::MemberUpdatedEvent, client| {
                    let handled_calls = handled_calls.clone();
                    async move {
                        handled_calls
                            .lock()
                            .expect("lock handled calls")
                            .push((event.event_type().to_owned(), client.base_url().to_owned()));
                    }
                },
            )
            .expect("register members.updated callback");

        let timestamp = current_timestamp();
        let typed_payload = member_payload("https://api.sumup.com", "members.updated");
        let typed_signature = signature_for(&secret, timestamp, &typed_payload);
        handler
            .handle(&typed_payload, typed_signature, timestamp.to_string())
            .await
            .expect("handle registered event");

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
            vec![(
                "members.updated".to_owned(),
                "https://api.sumup.com".to_owned()
            )]
        );
        assert_eq!(
            *unhandled.lock().expect("lock fallback calls"),
            vec![
                ("members.created".to_owned(), false),
                ("merchant.updated".to_owned(), true)
            ]
        );
        assert_eq!(handler.registered_event_types(), vec!["members.updated"]);
    }

    #[tokio::test]
    async fn event_notification_handler_verifies_before_dispatching() {
        let client = crate::Client::default();
        let secret = test_secret();
        let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let fallback_calls = calls.clone();
        let mut handler = client.event_notification_handler(&secret, move |_, _| {
            let fallback_calls = fallback_calls.clone();
            async move {
                fallback_calls.fetch_add(1, Ordering::Relaxed);
            }
        });
        let handled_calls = calls.clone();
        handler
            .on(move |_: crate::resources::members::MemberUpdatedEvent, _| {
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
    async fn event_notification_handler_rejects_duplicate_and_late_registration() {
        let client = crate::Client::default();
        let secret = test_secret();
        let mut handler = client.event_notification_handler(&secret, |_, _| async {});

        handler
            .on(|_: crate::resources::members::MemberUpdatedEvent, _| async {})
            .expect("register callback");
        let duplicate = handler
            .on(|_: crate::resources::members::MemberUpdatedEvent, _| async {})
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

        let late = handler
            .on(|_: crate::resources::readers::ReaderCreatedEvent, _| async {})
            .expect_err("reject late callback");
        assert_eq!(late, EventHandlerRegistrationError::RegistrationClosed);
    }

    #[tokio::test]
    async fn event_notification_handler_surfaces_callback_errors() {
        let client = crate::Client::default();
        let secret = test_secret();
        let mut handler = client.event_notification_handler(&secret, |_, _| async {});
        handler
            .on(
                |_: crate::resources::members::MemberUpdatedEvent, _| async {
                    Err::<(), _>(std::io::Error::other("callback failed"))
                },
            )
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
                assert!(matches!(result, Err(EventFetchError::InvalidObjectUrl(_))));
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
                assert!(matches!(result, Err(EventFetchError::InvalidObjectUrl(_))));
            }
            other => panic!("expected readers.created event, got {:?}", other),
        }
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
