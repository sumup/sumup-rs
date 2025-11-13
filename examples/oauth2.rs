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

use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use sumup::Client;

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

    let oauth_client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri)?);

    let (state, pkce_verifier) = handle_login(&oauth_client).await.unwrap();
    handle_callback(&oauth_client, state, pkce_verifier)
        .await
        .unwrap();

    Ok(())
}

/// Illustrates what your `/login` handler would look like.
async fn handle_login(
    oauth_client: &BasicClient,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Scope is a mechanism in OAuth 2.0 to limit an application's access to a user's account.
    // You should always request the minimal set of scope that you need for your application to
    // work. In this example we use "email profile" scope which gives you access to user's
    // email address and their profile.
    let scopes: Vec<Scope> = std::env::var("SUMUP_OAUTH_SCOPES")
        .unwrap_or_else(|_| "email profile".into())
        .split(',')
        .map(|scope| scope.trim())
        .filter(|scope| !scope.is_empty())
        .map(|scope| Scope::new(scope.into()))
        .collect();

    let (authorization_url, csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .add_scopes(scopes)
        .url();

    // Store these alongside your session/cookie data so the callback can resume the flow.
    let state = csrf_token.secret().to_owned();
    let pkce_verifier = pkce_verifier.secret().to_owned();

    println!("Redirect your users to: {authorization_url}");

    Ok((state, pkce_verifier))
}

/// Illustrates what your `/callback` would look like.
///
/// In a real application you would:
/// 1. Verify the incoming `state` matches what you stored for the user session.
/// 2. Recreate the PKCE verifier from your storage.
/// 3. Call this helper with the verified code + PKCE pair.
#[allow(dead_code)]
async fn handle_callback(
    oauth_client: &BasicClient,
    // authorization_code and pkce_verifier would in a deployment come from a secure session/cookie
    // state
    authorization_code: String,
    stored_pkce_verifier: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let token_response = oauth_client
        .exchange_code(AuthorizationCode::new(authorization_code))
        .set_pkce_verifier(PkceCodeVerifier::new(stored_pkce_verifier))
        .request_async(async_http_client)
        .await?;

    let access_token = token_response.access_token().secret().to_owned();
    let client = Client::default().with_authorization(access_token);

    let memberships = client
        .memberships()
        .list(Default::default())
        .await
        .expect("failed to get user's memberships");

    let merchant_code = match memberships.items.first() {
        Some(sumup::Membership { resource, .. }) => &resource.id,
        None => panic!("User doesn't have any memberships"),
    };

    println!("✔ Request authenticated, merchant code: `{merchant_code}`");

    Ok(())
}
