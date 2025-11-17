//! OAuth 2.0 Authorization Code flow with SumUp
//!
//! This example walks your through the steps necessary to implement
//! OAuth 2.0 (<https://oauth.net/>) in case you are building a software
//! for other people to use.
//!
//! To get started, you will need your client credentials.
//! If you don't have any yet, you can create them in the
//! [Developer Settings](https://me.sumup.com/en-us/settings/oauth2-applications).
//!
//! Your credentials need to be configured with the correct redirect URI,
//! that's the URI the user will get redirected to once they authenticate
//! and authorize your application. For development, you might want to
//! use for example `http://localhost:8080/callback`. In production, you would
//! redirect the user back to your host, e.g. `https://example.com/callback`.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
    routing::get,
    Json, Router, Server,
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

use sumup::{
    resources::merchants::{GetMerchantParams, Merchant},
    Client,
};

#[derive(Clone)]
struct AppState {
    oauth_client: Arc<BasicClient>,
    pkce_store: Arc<Mutex<HashMap<String, String>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID environment variable must be set");
    let client_secret =
        std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET environment variable must be set");
    let redirect_uri =
        std::env::var("REDIRECT_URI").expect("REDIRECT_URI environment variable must be set");

    let auth_url = AuthUrl::new("https://api.sumup.com/authorize".to_string())
        .expect("invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://api.sumup.com/token".to_string())
        .expect("invalid token endpoint URL");

    let oauth_client = Arc::new(
        BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri)?),
    );

    let app_state = AppState {
        oauth_client,
        pkce_store: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/login", get(handle_login))
        .route("/callback", get(handle_callback))
        .with_state(app_state.clone());

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    println!("Server is running at http://localhost:8080");

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

async fn handle_login(State(state): State<AppState>) -> Redirect {
    let state_token = Uuid::new_v4().to_string();
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    {
        // In-memory store used only for the purpose of this example.
        // Don't use in production. Normally you would rely on a proper
        // server-side sessions or encrypted cookie.
        let mut pkce_store = state.pkce_store.lock().await;
        pkce_store.insert(state_token.clone(), pkce_verifier.secret().to_owned());
    }

    let (authorization_url, _csrf_token) = state
        .oauth_client
        .authorize_url(move || CsrfToken::new(state_token.clone()))
        // Scope is a mechanism in OAuth 2.0 to limit an application's access to a user's account.
        // You should always request the minimal set of scope that you need for your application to
        // work. In this example we use "email profile" scope which gives you access to user's
        // email address and their profile.
        .add_scope(Scope::new("email profile".into()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    Redirect::temporary(authorization_url.as_str())
}

#[derive(Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
    merchant_code: Option<String>,
}

async fn handle_callback(
    State(state): State<AppState>,
    Query(params): Query<CallbackParams>,
) -> Result<Json<Merchant>, (StatusCode, String)> {
    let pkce_verifier = {
        let mut pkce_store = state.pkce_store.lock().await;
        pkce_store.remove(&params.state)
    }
    .ok_or_else(|| (StatusCode::BAD_REQUEST, "invalid oauth state".to_string()))?;

    let token_response = state
        .oauth_client
        .exchange_code(AuthorizationCode::new(params.code.clone()))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
        .request_async(async_http_client)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unable to retrieve token: {err}"),
            )
        })?;

    let merchant_code = params
        .merchant_code
        .clone()
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "missing merchant_code".to_string()))?;

    let access_token = token_response.access_token().secret().to_owned();
    let client = Client::default().with_authorization(access_token);

    println!("merchant Code: {merchant_code}");

    let merchant = client
        .merchants()
        .get(merchant_code, GetMerchantParams::default())
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("get merchant information: {err:?}"),
            )
        })?;

    Ok(Json(merchant))
}
