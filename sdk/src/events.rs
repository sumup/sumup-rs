// The contents of this file are generated; do not modify them.

//! Verify and parse event notifications sent by SumUp.
//!
//! Events let your integration react to changes in SumUp without polling the
//! API. Use them to update orders in your own system when a resource changes,
//! trigger fulfillment or accounting workflows, reconcile asynchronous state,
//! and keep local records in sync with SumUp.
//!
//! Event receivers should read the HTTP request body as raw bytes. Most
//! integrations can register typed async callbacks with the generated
//! `on_*` methods on [`EventsHandler`] and pass the body together with the
//! `X-SumUp-Webhook-Signature` and `X-SumUp-Webhook-Timestamp` headers to
//! [`EventsHandler::handle`]. Use [`crate::Client::parse_event_notification`]
//! when direct matching is a better fit. Both paths verify the signature
//! and timestamp before dispatching or returning an event.
pub(crate) use crate::event::RawEvent;
pub use crate::event::{
    DEFAULT_TOLERANCE, Event, EventCallbackError, EventError, EventHandlerRegistrationError,
    EventHandlingError, EventObject, EventSpec, EventsHandler, FetchObject, IntoEventHandlerResult,
    SIGNATURE_HEADER, SIGNATURE_VERSION, TIMESTAMP_HEADER, UnknownEvent, verify_signature,
};
impl EventsHandler {
    /// Registers an async callback for `members.created` notifications.
    ///
    /// The callback receives the typed event and a clone of the client.
    /// Returns an error if a callback is already registered for this event.
    pub fn on_member_created<HandlerFuture>(
        &mut self,
        callback: impl Fn(crate::resources::members::MemberCreatedEvent, crate::Client) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        HandlerFuture: std::future::Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        self.register::<crate::resources::members::MemberCreated, _>(callback)
    }
    /// Registers an async callback for `members.deleted` notifications.
    ///
    /// The callback receives the typed event and a clone of the client.
    /// Returns an error if a callback is already registered for this event.
    pub fn on_member_deleted<HandlerFuture>(
        &mut self,
        callback: impl Fn(crate::resources::members::MemberDeletedEvent, crate::Client) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        HandlerFuture: std::future::Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        self.register::<crate::resources::members::MemberDeleted, _>(callback)
    }
    /// Registers an async callback for `members.updated` notifications.
    ///
    /// The callback receives the typed event and a clone of the client.
    /// Returns an error if a callback is already registered for this event.
    pub fn on_member_updated<HandlerFuture>(
        &mut self,
        callback: impl Fn(crate::resources::members::MemberUpdatedEvent, crate::Client) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        HandlerFuture: std::future::Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        self.register::<crate::resources::members::MemberUpdated, _>(callback)
    }
    /// Registers an async callback for `readers.created` notifications.
    ///
    /// The callback receives the typed event and a clone of the client.
    /// Returns an error if a callback is already registered for this event.
    pub fn on_reader_created<HandlerFuture>(
        &mut self,
        callback: impl Fn(crate::resources::readers::ReaderCreatedEvent, crate::Client) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        HandlerFuture: std::future::Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        self.register::<crate::resources::readers::ReaderCreated, _>(callback)
    }
    /// Registers an async callback for `readers.deleted` notifications.
    ///
    /// The callback receives the typed event and a clone of the client.
    /// Returns an error if a callback is already registered for this event.
    pub fn on_reader_deleted<HandlerFuture>(
        &mut self,
        callback: impl Fn(crate::resources::readers::ReaderDeletedEvent, crate::Client) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        HandlerFuture: std::future::Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        self.register::<crate::resources::readers::ReaderDeleted, _>(callback)
    }
}
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
pub(crate) fn parse_known_event(client: crate::Client, event: RawEvent) -> EventNotification {
    match event.event_type() {
        <crate::resources::members::MemberCreated as EventSpec>::EVENT_TYPE => {
            EventNotification::MemberCreated(
                crate::resources::members::MemberCreatedEvent::from_raw(client, event),
            )
        }
        <crate::resources::members::MemberDeleted as EventSpec>::EVENT_TYPE => {
            EventNotification::MemberDeleted(
                crate::resources::members::MemberDeletedEvent::from_raw(client, event),
            )
        }
        <crate::resources::members::MemberUpdated as EventSpec>::EVENT_TYPE => {
            EventNotification::MemberUpdated(
                crate::resources::members::MemberUpdatedEvent::from_raw(client, event),
            )
        }
        <crate::resources::readers::ReaderCreated as EventSpec>::EVENT_TYPE => {
            EventNotification::ReaderCreated(
                crate::resources::readers::ReaderCreatedEvent::from_raw(client, event),
            )
        }
        <crate::resources::readers::ReaderDeleted as EventSpec>::EVENT_TYPE => {
            EventNotification::ReaderDeleted(
                crate::resources::readers::ReaderDeletedEvent::from_raw(client, event),
            )
        }
        _ => EventNotification::Unknown(UnknownEvent::from_raw(client, event)),
    }
}
