//! Event receiver example for SumUp events.
//!
//! This example shows the recommended event flow:
//! 1. read the raw request body
//! 2. verify `X-SumUp-Webhook-Signature` and `X-SumUp-Webhook-Timestamp`
//!    before parsing JSON
//! 3. parse the notification into a typed SDK event
//! 4. optionally resolve the thin event into the latest resource state
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
    events::{EventError, EventNotification, FetchObject, SIGNATURE_HEADER, TIMESTAMP_HEADER},
    Client, Secret,
};

#[derive(Clone)]
struct AppState {
    client: Client,
    event_secret: Secret,
}

#[tokio::main]
async fn main() {
    let event_secret = std::env::var("SUMUP_EVENT_SECRET")
        .expect("SUMUP_EVENT_SECRET environment variable must be set");

    let state = Arc::new(AppState {
        client: Client::default(),
        event_secret: Secret::new(event_secret),
    });

    let app = Router::new()
        .route("/events", post(handle_event))
        .with_state(state);

    println!("Listening for events on http://127.0.0.1:3000/events");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind event listener");
    axum::serve(listener, app)
        .await
        .expect("serve event listener");
}

async fn handle_event(
    State(state): State<Arc<AppState>>,
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

    let events_handler = state.client.events_handler(state.event_secret.secret());
    let event = match events_handler.parse(body.as_ref(), signature, timestamp) {
        Ok(event) => event,
        Err(
            EventError::MissingSignature
            | EventError::MissingTimestamp
            | EventError::InvalidSignature
            | EventError::SignatureExpired,
        ) => {
            return (StatusCode::UNAUTHORIZED, "invalid signature").into_response();
        }
        Err(
            EventError::InvalidSignatureHeader
            | EventError::InvalidTimestampHeader(_)
            | EventError::InvalidPayload(_),
        ) => {
            return (StatusCode::BAD_REQUEST, "invalid event payload").into_response();
        }
    };

    match &event {
        EventNotification::MemberCreated(event) => {
            println!(
                "received {} for member {}",
                event.event_type(),
                event.object.id
            );
        }
        EventNotification::MemberUpdated(event) => {
            println!(
                "received {} for member {}",
                event.event_type(),
                event.object.id
            );
        }
        EventNotification::MemberDeleted(event) => {
            println!(
                "received {} for member {}",
                event.event_type(),
                event.object.id
            );
        }
        EventNotification::ReaderCreated(event) => {
            println!(
                "received {} for reader {}",
                event.event_type(),
                event.object.id
            );
        }
        EventNotification::ReaderDeleted(event) => {
            println!(
                "received {} for reader {}",
                event.event_type(),
                event.object.id
            );
        }
        EventNotification::Unknown(event) => {
            println!("received unsupported event type {}", event.event_type);
            return StatusCode::ACCEPTED.into_response();
        }
        _ => {
            println!("received unsupported event type {}", event.event_type());
            return StatusCode::ACCEPTED.into_response();
        }
    }

    // The event payload currently looks thin, so resolving it into the latest
    // API representation is often the most reliable way to continue processing.
    if state.client.authorization().is_some() {
        match event {
            EventNotification::MemberCreated(event) => log_member(event.fetch_object().await),
            EventNotification::MemberDeleted(event) => log_member(event.fetch_object().await),
            EventNotification::MemberUpdated(event) => log_member(event.fetch_object().await),
            EventNotification::ReaderCreated(event) => log_reader(event.fetch_object().await),
            EventNotification::ReaderDeleted(event) => log_reader(event.fetch_object().await),
            EventNotification::Unknown(_) => {}
            _ => {}
        }
    } else {
        println!("SUMUP_API_KEY not set, skipping resource fetch");
    }

    StatusCode::OK.into_response()
}

fn log_member(result: Result<sumup::members::Member, sumup::events::EventFetchError>) {
    match result {
        Ok(member) => {
            println!("latest member status: {:?}", member.status);
        }
        Err(err) => {
            eprintln!("failed to fetch latest member state: {}", err);
        }
    }
}

fn log_reader(result: Result<sumup::readers::Reader, sumup::events::EventFetchError>) {
    match result {
        Ok(reader) => {
            println!("latest reader status: {:?}", reader.status);
        }
        Err(err) => {
            eprintln!("failed to fetch latest reader state: {}", err);
        }
    }
}
