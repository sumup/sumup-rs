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
//! `X-SumUp-Webhook-Signature` header to
//! [`EventsHandler::handle`]. Use [`crate::Client::parse_event_notification`]
//! when direct matching is a better fit. Both paths verify the signature
//! and enforces a fixed five-minute clock skew before dispatching or returning an event.
pub(crate) use crate::event::RawEvent;
pub use crate::event::{
    Event, EventCallbackError, EventError, EventHandlerRegistrationError, EventHandlingError,
    EventObject, EventSpec, EventsHandler, FetchObject, IntoEventHandlerResult, SIGNATURE_HEADER,
    SIGNATURE_VERSION, UnknownEvent, verify_signature,
};
impl EventsHandler {
    /// Registers an async callback for `checkouts.created` notifications.
    ///
    /// The callback receives the typed event and a clone of the client.
    /// Returns an error if a callback is already registered for this event.
    pub fn on_checkout_created<HandlerFuture>(
        &mut self,
        callback: impl Fn(
            crate::resources::checkouts::CheckoutCreatedEvent,
            crate::Client,
        ) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        HandlerFuture: std::future::Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        self.register::<crate::resources::checkouts::CheckoutCreated, _>(callback)
    }
    /// Registers an async callback for `checkouts.expired` notifications.
    ///
    /// The callback receives the typed event and a clone of the client.
    /// Returns an error if a callback is already registered for this event.
    pub fn on_checkout_expired<HandlerFuture>(
        &mut self,
        callback: impl Fn(
            crate::resources::checkouts::CheckoutExpiredEvent,
            crate::Client,
        ) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        HandlerFuture: std::future::Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        self.register::<crate::resources::checkouts::CheckoutExpired, _>(callback)
    }
    /// Registers an async callback for `checkouts.failed` notifications.
    ///
    /// The callback receives the typed event and a clone of the client.
    /// Returns an error if a callback is already registered for this event.
    pub fn on_checkout_failed<HandlerFuture>(
        &mut self,
        callback: impl Fn(
            crate::resources::checkouts::CheckoutFailedEvent,
            crate::Client,
        ) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        HandlerFuture: std::future::Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        self.register::<crate::resources::checkouts::CheckoutFailed, _>(callback)
    }
    /// Registers an async callback for `checkouts.paid` notifications.
    ///
    /// The callback receives the typed event and a clone of the client.
    /// Returns an error if a callback is already registered for this event.
    pub fn on_checkout_paid<HandlerFuture>(
        &mut self,
        callback: impl Fn(
            crate::resources::checkouts::CheckoutPaidEvent,
            crate::Client,
        ) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        HandlerFuture: std::future::Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        self.register::<crate::resources::checkouts::CheckoutPaid, _>(callback)
    }
    /// Registers an async callback for `checkouts.updated` notifications.
    ///
    /// The callback receives the typed event and a clone of the client.
    /// Returns an error if a callback is already registered for this event.
    pub fn on_checkout_updated<HandlerFuture>(
        &mut self,
        callback: impl Fn(
            crate::resources::checkouts::CheckoutUpdatedEvent,
            crate::Client,
        ) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        HandlerFuture: std::future::Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        self.register::<crate::resources::checkouts::CheckoutUpdated, _>(callback)
    }
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
    /// Registers an async callback for `roles.created` notifications.
    ///
    /// The callback receives the typed event and a clone of the client.
    /// Returns an error if a callback is already registered for this event.
    pub fn on_role_created<HandlerFuture>(
        &mut self,
        callback: impl Fn(crate::resources::roles::RoleCreatedEvent, crate::Client) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        HandlerFuture: std::future::Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        self.register::<crate::resources::roles::RoleCreated, _>(callback)
    }
    /// Registers an async callback for `roles.deleted` notifications.
    ///
    /// The callback receives the typed event and a clone of the client.
    /// Returns an error if a callback is already registered for this event.
    pub fn on_role_deleted<HandlerFuture>(
        &mut self,
        callback: impl Fn(crate::resources::roles::RoleDeletedEvent, crate::Client) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        HandlerFuture: std::future::Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        self.register::<crate::resources::roles::RoleDeleted, _>(callback)
    }
    /// Registers an async callback for `roles.updated` notifications.
    ///
    /// The callback receives the typed event and a clone of the client.
    /// Returns an error if a callback is already registered for this event.
    pub fn on_role_updated<HandlerFuture>(
        &mut self,
        callback: impl Fn(crate::resources::roles::RoleUpdatedEvent, crate::Client) -> HandlerFuture
        + Send
        + Sync
        + 'static,
    ) -> Result<&mut Self, EventHandlerRegistrationError>
    where
        HandlerFuture: std::future::Future + Send + 'static,
        HandlerFuture::Output: IntoEventHandlerResult + 'static,
    {
        self.register::<crate::resources::roles::RoleUpdated, _>(callback)
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
    CheckoutCreated(crate::resources::checkouts::CheckoutCreatedEvent),
    CheckoutExpired(crate::resources::checkouts::CheckoutExpiredEvent),
    CheckoutFailed(crate::resources::checkouts::CheckoutFailedEvent),
    CheckoutPaid(crate::resources::checkouts::CheckoutPaidEvent),
    CheckoutUpdated(crate::resources::checkouts::CheckoutUpdatedEvent),
    MemberCreated(crate::resources::members::MemberCreatedEvent),
    MemberDeleted(crate::resources::members::MemberDeletedEvent),
    MemberUpdated(crate::resources::members::MemberUpdatedEvent),
    ReaderCreated(crate::resources::readers::ReaderCreatedEvent),
    ReaderDeleted(crate::resources::readers::ReaderDeletedEvent),
    RoleCreated(crate::resources::roles::RoleCreatedEvent),
    RoleDeleted(crate::resources::roles::RoleDeletedEvent),
    RoleUpdated(crate::resources::roles::RoleUpdatedEvent),
    Unknown(UnknownEvent),
}
impl EventNotification {
    /// Returns the event type string, such as `members.updated`.
    pub fn event_type(&self) -> &str {
        match self {
            Self::CheckoutCreated(event) => event.event_type(),
            Self::CheckoutExpired(event) => event.event_type(),
            Self::CheckoutFailed(event) => event.event_type(),
            Self::CheckoutPaid(event) => event.event_type(),
            Self::CheckoutUpdated(event) => event.event_type(),
            Self::MemberCreated(event) => event.event_type(),
            Self::MemberDeleted(event) => event.event_type(),
            Self::MemberUpdated(event) => event.event_type(),
            Self::ReaderCreated(event) => event.event_type(),
            Self::ReaderDeleted(event) => event.event_type(),
            Self::RoleCreated(event) => event.event_type(),
            Self::RoleDeleted(event) => event.event_type(),
            Self::RoleUpdated(event) => event.event_type(),
            Self::Unknown(event) => &event.event_type,
        }
    }
}
pub(crate) fn parse_known_event(client: crate::Client, event: RawEvent) -> EventNotification {
    match event.event_type() {
        <crate::resources::checkouts::CheckoutCreated as EventSpec>::EVENT_TYPE => {
            EventNotification::CheckoutCreated(
                crate::resources::checkouts::CheckoutCreatedEvent::from_raw(client, event),
            )
        }
        <crate::resources::checkouts::CheckoutExpired as EventSpec>::EVENT_TYPE => {
            EventNotification::CheckoutExpired(
                crate::resources::checkouts::CheckoutExpiredEvent::from_raw(client, event),
            )
        }
        <crate::resources::checkouts::CheckoutFailed as EventSpec>::EVENT_TYPE => {
            EventNotification::CheckoutFailed(
                crate::resources::checkouts::CheckoutFailedEvent::from_raw(client, event),
            )
        }
        <crate::resources::checkouts::CheckoutPaid as EventSpec>::EVENT_TYPE => {
            EventNotification::CheckoutPaid(
                crate::resources::checkouts::CheckoutPaidEvent::from_raw(client, event),
            )
        }
        <crate::resources::checkouts::CheckoutUpdated as EventSpec>::EVENT_TYPE => {
            EventNotification::CheckoutUpdated(
                crate::resources::checkouts::CheckoutUpdatedEvent::from_raw(client, event),
            )
        }
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
        <crate::resources::roles::RoleCreated as EventSpec>::EVENT_TYPE => {
            EventNotification::RoleCreated(crate::resources::roles::RoleCreatedEvent::from_raw(
                client, event,
            ))
        }
        <crate::resources::roles::RoleDeleted as EventSpec>::EVENT_TYPE => {
            EventNotification::RoleDeleted(crate::resources::roles::RoleDeletedEvent::from_raw(
                client, event,
            ))
        }
        <crate::resources::roles::RoleUpdated as EventSpec>::EVENT_TYPE => {
            EventNotification::RoleUpdated(crate::resources::roles::RoleUpdatedEvent::from_raw(
                client, event,
            ))
        }
        _ => EventNotification::Unknown(UnknownEvent::from_raw(client, event)),
    }
}
