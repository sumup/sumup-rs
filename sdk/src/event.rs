//! Verify and parse event notifications sent by SumUp.
//!
//! Events let your integration react to changes in SumUp without polling the
//! API. Use them to update orders in your own system when a checkout is paid,
//! failed, or expired, trigger fulfillment or accounting workflows, reconcile
//! asynchronous payment state, and keep local records in sync with SumUp.
//!
//! Event receivers should read the HTTP request body as raw bytes and pass it
//! together with the `X-SumUp-Webhook-Signature` and
//! `X-SumUp-Webhook-Timestamp` headers to [`EventsHandler::parse`]. The SDK
//! verifies the signature and timestamp before deserializing the payload.
//!
//! ```no_run
//! # use sumup::{Client, Secret};
//! # use sumup::events::{EventNotification, SIGNATURE_HEADER, TIMESTAMP_HEADER};
//! # use axum::{body::Bytes, http::HeaderMap};
//! # fn header<'a>(headers: &'a HeaderMap, name: &str) -> &'a str {
//! #     headers.get(name).and_then(|value| value.to_str().ok()).unwrap()
//! # }
//! # fn example(headers: HeaderMap, body: Bytes, secret: Secret) -> Result<(), sumup::events::EventError> {
//! let client = Client::default();
//! let event = client
//!     .events_handler(secret.secret())
//!     .parse(
//!         body.as_ref(),
//!         header(&headers, SIGNATURE_HEADER),
//!         header(&headers, TIMESTAMP_HEADER),
//!     )?;
//!
//! match event {
//!     EventNotification::MemberUpdated(event) => {
//!         println!("member updated: {}", event.object.id);
//!     }
//!     EventNotification::Unknown(event) => {
//!         println!("unknown event type: {}", event.event_type);
//!     }
//!     _ => {}
//! }
//! # Ok(())
//! # }
//! ```
//!
//! Parsed notifications include a compact reference to the affected API
//! resource. Known event types expose [`FetchObject::fetch_object`] so your
//! integration can load the latest checkout or member state before continuing
//! domain-specific processing. Event delivery can be retried, so handlers should
//! be idempotent and safe to run more than once.

pub use crate::events_handler::{
    verify_signature, EventError, EventFetchError, EventsHandler, DEFAULT_TOLERANCE,
    SIGNATURE_HEADER, SIGNATURE_VERSION, TIMESTAMP_HEADER,
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct RawObject {
    id: String,
    #[serde(rename = "type")]
    object_type: String,
    url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct RawEvent {
    id: String,
    #[serde(rename = "type")]
    event_type: String,
    created_at: crate::datetime::DateTime,
    object: RawObject,
}

impl RawEvent {
    pub(crate) fn event_type(&self) -> &str {
        &self.event_type
    }
}

/// Reference to the API resource affected by an event.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventObject {
    /// Resource identifier from the event payload.
    pub id: String,
    /// Resource type from the event payload.
    #[serde(rename = "type")]
    pub object_type: String,
    /// API URL that can be used to fetch the resource.
    pub url: String,
}

impl From<RawObject> for EventObject {
    fn from(value: RawObject) -> Self {
        Self {
            id: value.id,
            object_type: value.object_type,
            url: value.url,
        }
    }
}

/// Trait implemented by typed events that can fetch their referenced API
/// resource.
pub trait FetchObject {
    /// SDK type returned when the referenced resource is fetched.
    type Object;

    /// Fetches the latest representation of the referenced API resource.
    fn fetch_object(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Object, EventFetchError>> + '_;
}

/// Static metadata for a typed event notification.
pub trait EventSpec {
    /// Event type string from the notification payload.
    const EVENT_TYPE: &'static str;
    /// Expected object type referenced by this event.
    const OBJECT_TYPE: &'static str;

    /// SDK type returned when the referenced resource is fetched.
    type FetchedObject: serde::de::DeserializeOwned;
}

/// Event notification with static event metadata.
#[derive(Debug, Clone)]
pub struct Event<'a, E: EventSpec> {
    /// Event identifier from the notification.
    pub id: String,
    /// Time when the event was created.
    pub created_at: crate::datetime::DateTime,
    /// Referenced API resource.
    pub object: EventObject,
    client: &'a crate::Client,
    spec: std::marker::PhantomData<E>,
}

impl<'a, E: EventSpec> Event<'a, E> {
    pub(crate) fn from_raw(client: &'a crate::Client, event: RawEvent) -> Self {
        Self {
            id: event.id,
            created_at: event.created_at,
            object: event.object.into(),
            client,
            spec: std::marker::PhantomData,
        }
    }

    /// Returns the event type.
    pub fn event_type(&self) -> &'static str {
        E::EVENT_TYPE
    }
}

impl<E: EventSpec> FetchObject for Event<'_, E> {
    type Object = E::FetchedObject;

    fn fetch_object(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Object, EventFetchError>> + '_ {
        crate::events_handler::fetch_object(self.client, &self.object.url)
    }
}

/// Event notification for an event type that does not yet have a dedicated SDK
/// variant.
#[derive(Debug, Clone)]
pub struct UnknownEvent<'a> {
    /// Event identifier from the notification.
    pub id: String,
    /// Event type string from the notification.
    pub event_type: String,
    /// Time when the event was created.
    pub created_at: crate::datetime::DateTime,
    /// Referenced API resource.
    pub object: EventObject,
    client: &'a crate::Client,
}

impl<'a> UnknownEvent<'a> {
    pub(crate) fn from_raw(client: &'a crate::Client, event: RawEvent) -> Self {
        Self {
            id: event.id,
            event_type: event.event_type,
            created_at: event.created_at,
            object: event.object.into(),
            client,
        }
    }
}

impl<'a> FetchObject for UnknownEvent<'a> {
    type Object = serde_json::Value;

    fn fetch_object(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Object, EventFetchError>> + '_ {
        crate::events_handler::fetch_object(self.client, &self.object.url)
    }
}
