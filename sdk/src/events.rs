// The contents of this file are generated; do not modify them.

//! Verify and parse event notifications sent by SumUp.
//!
//! Events let your integration react to changes in SumUp without polling the
//! API. Use them to update orders in your own system when a resource changes,
//! trigger fulfillment or accounting workflows, reconcile asynchronous state,
//! and keep local records in sync with SumUp.
//!
//! Event receivers should read the HTTP request body as raw bytes. Most
//! integrations can register typed async callbacks with
//! [`EventsHandler::on`] and pass the body together with the
//! `X-SumUp-Webhook-Signature` and `X-SumUp-Webhook-Timestamp` headers to
//! [`EventsHandler::handle`]. Use [`crate::Client::parse_event_notification`]
//! when direct matching is a better fit. Both paths verify the signature
//! and timestamp before dispatching or returning an event.
pub(crate) use crate::event::RawEvent;
pub use crate::event::{
    verify_signature, Event, EventCallbackError, EventError, EventFetchError,
    EventHandlerRegistrationError, EventHandlingError, EventObject, EventSpec, EventsHandler,
    FetchObject, IntoEventHandlerResult, UnknownEvent, DEFAULT_TOLERANCE, SIGNATURE_HEADER,
    SIGNATURE_VERSION, TIMESTAMP_HEADER,
};
/// Event notification parsed by the SDK.
///
/// Known event types are represented by dedicated variants. Unknown event types
/// are preserved as [`EventNotification::Unknown`] so your integration can
/// safely acknowledge or log them without losing the raw event type and object
/// reference.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EventNotification {
    MemberCreated(crate::resources::members::MemberCreatedEvent),
    MemberDeleted(crate::resources::members::MemberDeletedEvent),
    MemberUpdated(crate::resources::members::MemberUpdatedEvent),
    ReaderCreated(crate::resources::readers::ReaderCreatedEvent),
    ReaderDeleted(crate::resources::readers::ReaderDeletedEvent),
    Unknown(UnknownEvent),
}
impl EventNotification {
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
pub(crate) fn parse_known_event(
    client: crate::Client,
    event: RawEvent,
) -> Result<EventNotification, EventError> {
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
