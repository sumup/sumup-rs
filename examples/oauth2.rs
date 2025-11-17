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
    response::Redirect,
    routing::get,
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use oauth2::reqwest;
use oauth2::{basic::BasicClient, EndpointNotSet, EndpointSet};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use sumup::{
    resources::merchants::{GetMerchantParams, Merchant},
    Client,
};

#[derive(Clone)]
struct AppState {
    oauth_client:
        Arc<BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>>,
}

const STATE_COOKIE_NAME: &str = "oauth_state";
const PKCE_COOKIE_NAME: &str = "oauth_pkce";

#[tokio::main]
async fn main() {
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
        BasicClient::new(ClientId::new(client_id))
            .set_client_secret(ClientSecret::new(client_secret))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap()),
    );

    let app_state = AppState { oauth_client };

    let app = Router::new()
        .route("/login", get(handle_login))
        .route("/callback", get(handle_callback))
        .with_state(app_state.clone());

    println!("Server is running at http://localhost:8080");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn handle_login(State(state): State<AppState>, jar: CookieJar) -> (CookieJar, Redirect) {
    let state_token = Uuid::new_v4().to_string();
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let state_cookie = Cookie::build((STATE_COOKIE_NAME, state_token.clone()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        // Set to true on production when running on https
        .secure(false);

    let pkce_cookie = Cookie::build((PKCE_COOKIE_NAME, pkce_verifier.secret().to_owned()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        // Set to true on production when running on https
        .secure(false);

    let jar = jar.add(state_cookie).add(pkce_cookie);

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

    (jar, Redirect::temporary(authorization_url.as_str()))
}

#[derive(Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
    merchant_code: Option<String>,
}

async fn handle_callback(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(params): Query<CallbackParams>,
) -> (CookieJar, Json<Merchant>) {
    let state_cookie = jar
        .get(STATE_COOKIE_NAME)
        .map(|cookie| cookie.value().to_owned());
    let pkce_cookie = jar
        .get(PKCE_COOKIE_NAME)
        .map(|cookie| cookie.value().to_owned());

    let jar = jar.remove(Cookie::build(STATE_COOKIE_NAME));
    let jar = jar.remove(Cookie::build(PKCE_COOKIE_NAME));

    let state_cookie = state_cookie.expect("missing oauth state cookie");

    if params.state != state_cookie {
        panic!("invalid state cookie")
    }

    let pkce_verifier = PkceCodeVerifier::new(pkce_cookie.expect("missing oauth pkce cookie"));

    let token_response = state
        .oauth_client
        .exchange_code(AuthorizationCode::new(params.code.clone()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&reqwest::Client::default())
        .await
        .expect("retrieve token vie code exchange");

    // Users might have access to multiple merchant accounts, the `merchant_code` parameter
    // returned in the callback is the merchant code of their default merchant account.
    // In production, you would want to let users pick which merchant they want to use
    // using the memberships API.
    let merchant_code = params.merchant_code.expect("missing merchant code param");

    println!("merchant code: {merchant_code}");

    let access_token = token_response.access_token().secret().to_owned();
    let client = Client::default().with_authorization(access_token);

    let merchant = client
        .merchants()
        .get(merchant_code, GetMerchantParams::default())
        .await
        .expect("get mercahnt");

    (jar, Json(merchant))
}
