// The contents of this file are generated; do not modify them.

//! Verify and parse event notifications sent by SumUp.
//!
//! Events let your integration react to changes in SumUp without polling the
//! API. Use them to update orders in your own system when a resource changes,
//! trigger fulfillment or accounting workflows, reconcile asynchronous state,
//! and keep local records in sync with SumUp.
//!
//! Event receivers should read the HTTP request body as raw bytes and pass it
//! together with the `X-SumUp-Webhook-Signature` and
//! `X-SumUp-Webhook-Timestamp` headers to [`EventsHandler::parse`]. The SDK
//! verifies the signature and timestamp before deserializing the payload.
pub(crate) use crate::event::RawEvent;
pub use crate::event::{
    verify_signature, Event, EventError, EventFetchError, EventObject, EventSpec, EventsHandler,
    FetchObject, UnknownEvent, DEFAULT_TOLERANCE, SIGNATURE_HEADER, SIGNATURE_VERSION,
    TIMESTAMP_HEADER,
};
/// Event notification parsed by the SDK.
///
/// Known event types are represented by dedicated variants. Unknown event types
/// are preserved as [`EventNotification::Unknown`] so your integration can
/// safely acknowledge or log them without losing the raw event type and object
/// reference.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EventNotification<'a> {
    MemberCreated(crate::resources::members::MemberCreatedEvent<'a>),
    MemberDeleted(crate::resources::members::MemberDeletedEvent<'a>),
    MemberUpdated(crate::resources::members::MemberUpdatedEvent<'a>),
    ReaderCreated(crate::resources::readers::ReaderCreatedEvent<'a>),
    ReaderDeleted(crate::resources::readers::ReaderDeletedEvent<'a>),
    Unknown(UnknownEvent<'a>),
}
impl EventNotification<'_> {
    /// Returns the event type string, such as `members.updated`.
    pub fn event_type(&self) -> &str {
        match self {
            Self::MemberCreated(event) => event.event_type(),
            Self::MemberDeleted(event) => event.event_type(),
            Self::MemberUpdated(event) => event.event_type(),
            Self::ReaderCreated(event) => event.event_type(),
            Self::ReaderDeleted(event) => event.event_type(),
            Self::Unknown(event) => &event.event_type,
        }
    }
}
pub(crate) fn parse_known_event<'a>(
    client: &'a crate::Client,
    event: RawEvent,
) -> Result<EventNotification<'a>, EventError> {
    match event.event_type() {
        <crate::resources::members::MemberCreated as EventSpec>::EVENT_TYPE => {
            Ok(EventNotification::MemberCreated(
                crate::resources::members::MemberCreatedEvent::from_raw(client, event),
            ))
        }
        <crate::resources::members::MemberDeleted as EventSpec>::EVENT_TYPE => {
            Ok(EventNotification::MemberDeleted(
                crate::resources::members::MemberDeletedEvent::from_raw(client, event),
            ))
        }
        <crate::resources::members::MemberUpdated as EventSpec>::EVENT_TYPE => {
            Ok(EventNotification::MemberUpdated(
                crate::resources::members::MemberUpdatedEvent::from_raw(client, event),
            ))
        }
        <crate::resources::readers::ReaderCreated as EventSpec>::EVENT_TYPE => {
            Ok(EventNotification::ReaderCreated(
                crate::resources::readers::ReaderCreatedEvent::from_raw(client, event),
            ))
        }
        <crate::resources::readers::ReaderDeleted as EventSpec>::EVENT_TYPE => {
            Ok(EventNotification::ReaderDeleted(
                crate::resources::readers::ReaderDeletedEvent::from_raw(client, event),
            ))
        }
        _ => Ok(EventNotification::Unknown(UnknownEvent::from_raw(
            client, event,
        ))),
    }
}
