//! Event receiver example for SumUp events.
//!
//! This example shows the recommended event flow:
//! 1. register typed async callbacks and an unhandled-event fallback
//! 2. read the raw request body
//! 3. verify `X-SumUp-Webhook-Signature` and `X-SumUp-Webhook-Timestamp`
//! 4. route the notification to the matching callback
//! 5. optionally resolve the thin event into the latest resource state
//!
//! To run this example:
//! 1. Set your event secret:
//!    `export SUMUP_EVENT_SECRET="your_event_secret"`
//! 2. Optional: set your API key if you want to resolve the latest resource state:
//!    `export SUMUP_API_KEY="your_api_key"`
//! 3. Run:
//!    `cargo run --example events`
//! 4. Send a test request to:
//!    `http://127.0.0.1:3000/events`

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use std::sync::Arc;
use sumup::{
    events::{
        EventError, EventFetchError, EventHandlingError, EventNotification,
        EventNotificationHandler, FetchObject, SIGNATURE_HEADER, TIMESTAMP_HEADER,
    },
    members::MemberUpdatedEvent,
    readers::ReaderCreatedEvent,
    Client, Secret,
};

#[tokio::main]
async fn main() {
    let event_secret = std::env::var("SUMUP_EVENT_SECRET")
        .expect("SUMUP_EVENT_SECRET environment variable must be set");
    let event_secret = Secret::new(event_secret);
    let client = Client::default();
    let mut event_handler =
        client.event_notification_handler(event_secret.secret(), handle_unhandled_event);
    event_handler
        .on(handle_member_updated)
        .expect("register members.updated callback");
    event_handler
        .on(handle_reader_created)
        .expect("register readers.created callback");

    let app = Router::new()
        .route("/events", post(handle_event))
        .with_state(Arc::new(event_handler));

    println!("Listening for events on http://127.0.0.1:3000/events");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind event listener");
    axum::serve(listener, app)
        .await
        .expect("serve event listener");
}

async fn handle_event(
    State(handler): State<Arc<EventNotificationHandler>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let Some(signature) = headers.get(SIGNATURE_HEADER) else {
        return (StatusCode::BAD_REQUEST, "missing signature header").into_response();
    };
    let signature = match signature.to_str() {
        Ok(value) => value,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid signature header").into_response(),
    };

    let Some(timestamp) = headers.get(TIMESTAMP_HEADER) else {
        return (StatusCode::BAD_REQUEST, "missing timestamp header").into_response();
    };
    let timestamp = match timestamp.to_str() {
        Ok(value) => value,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid timestamp header").into_response(),
    };

    match handler.handle(body.as_ref(), signature, timestamp).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(EventHandlingError::Event(
            EventError::MissingSignature
            | EventError::MissingTimestamp
            | EventError::InvalidSignature
            | EventError::SignatureExpired,
        )) => (StatusCode::UNAUTHORIZED, "invalid signature").into_response(),
        Err(EventHandlingError::Event(
            EventError::InvalidSignatureHeader
            | EventError::InvalidTimestampHeader(_)
            | EventError::InvalidPayload(_),
        )) => (StatusCode::BAD_REQUEST, "invalid event payload").into_response(),
        Err(EventHandlingError::Callback(err)) => {
            eprintln!("event callback failed: {err}");
            (StatusCode::INTERNAL_SERVER_ERROR, "event callback failed").into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "event handling failed").into_response(),
    }
}

async fn handle_member_updated(
    event: MemberUpdatedEvent,
    client: Client,
) -> Result<(), EventFetchError> {
    println!(
        "received {} for member {}",
        event.event_type(),
        event.object.id
    );
    if client.authorization().is_none() {
        println!("SUMUP_API_KEY not set, skipping resource fetch");
        return Ok(());
    }

    let member = event.fetch_object().await?;
    println!("latest member status: {:?}", member.status);
    Ok(())
}

async fn handle_reader_created(
    event: ReaderCreatedEvent,
    client: Client,
) -> Result<(), EventFetchError> {
    println!(
        "received {} for reader {}",
        event.event_type(),
        event.object.id
    );
    if client.authorization().is_none() {
        println!("SUMUP_API_KEY not set, skipping resource fetch");
        return Ok(());
    }

    let reader = event.fetch_object().await?;
    println!("latest reader status: {:?}", reader.status);
    Ok(())
}

async fn handle_unhandled_event(event: EventNotification, _client: Client) {
    println!("received unhandled event type {}", event.event_type());
}
