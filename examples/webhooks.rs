//! Webhook receiver example for SumUp checkout events.
//!
//! This example shows the recommended webhook flow:
//! 1. read the raw request body
//! 2. verify `X-SumUp-Webhook-Signature` and `X-SumUp-Webhook-Timestamp`
//!    before parsing JSON
//! 3. parse the notification into a typed SDK event
//! 4. optionally resolve the thin webhook into the latest checkout state
//!
//! To run this example:
//! 1. Set your webhook secret:
//!    `export SUMUP_WEBHOOK_SECRET="your_webhook_secret"`
//! 2. Optional: set your API key if you want to resolve the latest checkout state:
//!    `export SUMUP_API_KEY="your_api_key"`
//! 3. Run:
//!    `cargo run --example webhooks`
//! 4. Send a test request to:
//!    `http://127.0.0.1:3000/webhooks`

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
    webhooks::{EventNotification, FetchObject, WebhookError, SIGNATURE_HEADER, TIMESTAMP_HEADER},
    Client, Secret,
};

#[derive(Clone)]
struct AppState {
    client: Client,
    webhook_secret: Secret,
}

#[tokio::main]
async fn main() {
    let webhook_secret = std::env::var("SUMUP_WEBHOOK_SECRET")
        .expect("SUMUP_WEBHOOK_SECRET environment variable must be set");

    let state = Arc::new(AppState {
        client: Client::default(),
        webhook_secret: Secret::new(webhook_secret),
    });

    let app = Router::new()
        .route("/webhooks", post(handle_webhook))
        .with_state(state);

    println!("Listening for webhooks on http://127.0.0.1:3000/webhooks");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind webhook listener");
    axum::serve(listener, app)
        .await
        .expect("serve webhook listener");
}

async fn handle_webhook(
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

    let webhook_handler = state.client.webhook_handler(state.webhook_secret.secret());
    let event = match webhook_handler.verify_and_parse(body.as_ref(), signature, timestamp) {
        Ok(event) => event,
        Err(
            WebhookError::MissingSignature
            | WebhookError::MissingTimestamp
            | WebhookError::InvalidSignature
            | WebhookError::SignatureExpired,
        ) => {
            return (StatusCode::UNAUTHORIZED, "invalid signature").into_response();
        }
        Err(
            WebhookError::InvalidSignatureHeader
            | WebhookError::InvalidTimestampHeader(_)
            | WebhookError::InvalidPayload(_)
            | WebhookError::UnexpectedObjectType { .. },
        ) => {
            return (StatusCode::BAD_REQUEST, "invalid webhook payload").into_response();
        }
    };

    match &event {
        EventNotification::CheckoutCreated(event) => {
            println!(
                "received {} for checkout {}",
                event.event_type(),
                event.object.id
            );
        }
        EventNotification::CheckoutUpdated(event) => {
            println!(
                "received {} for checkout {}",
                event.event_type(),
                event.object.id
            );
        }
        EventNotification::CheckoutPaid(event) => {
            println!(
                "received {} for checkout {}",
                event.event_type(),
                event.object.id
            );
        }
        EventNotification::CheckoutFailed(event) => {
            println!(
                "received {} for checkout {}",
                event.event_type(),
                event.object.id
            );
        }
        EventNotification::CheckoutExpired(event) => {
            println!(
                "received {} for checkout {}",
                event.event_type(),
                event.object.id
            );
        }
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
        EventNotification::Unknown(event) => {
            println!(
                "received unsupported webhook event type {}",
                event.event_type
            );
            return StatusCode::ACCEPTED.into_response();
        }
    }

    // The webhook payload currently looks thin, so resolving it into the latest
    // API representation is often the most reliable way to continue processing.
    if state.client.authorization().is_some() {
        match event {
            EventNotification::CheckoutCreated(event) => log_checkout(event.fetch_object().await),
            EventNotification::CheckoutUpdated(event) => log_checkout(event.fetch_object().await),
            EventNotification::CheckoutPaid(event) => log_checkout(event.fetch_object().await),
            EventNotification::CheckoutFailed(event) => log_checkout(event.fetch_object().await),
            EventNotification::CheckoutExpired(event) => log_checkout(event.fetch_object().await),
            EventNotification::MemberCreated(event) => log_member(event.fetch_object().await),
            EventNotification::MemberUpdated(event) => log_member(event.fetch_object().await),
            EventNotification::Unknown(_) => {}
        }
    } else {
        println!("SUMUP_API_KEY not set, skipping checkout fetch");
    }

    StatusCode::OK.into_response()
}

fn log_checkout(
    result: Result<sumup::checkouts::CheckoutSuccess, sumup::webhooks::WebhookFetchError>,
) {
    match result {
        Ok(checkout) => {
            println!(
                "latest checkout status: {}",
                checkout.status.as_deref().unwrap_or("unknown")
            );
        }
        Err(err) => {
            eprintln!("failed to fetch latest checkout state: {}", err);
        }
    }
}

fn log_member(result: Result<sumup::members::Member, sumup::webhooks::WebhookFetchError>) {
    match result {
        Ok(member) => {
            println!("latest member status: {:?}", member.status);
        }
        Err(err) => {
            eprintln!("failed to fetch latest member state: {}", err);
        }
    }
}
