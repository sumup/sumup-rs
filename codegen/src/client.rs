use heck::{ToSnakeCase, ToUpperCamelCase};
use openapiv3::OpenAPI;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::collections::HashMap;
use std::path::Path;

use crate::TagSchemas;

/// Writes the top-level API client file and tag accessors to the output directory.
pub fn generate_client_file(
    out_path: &Path,
    spec: &OpenAPI,
    tag_schemas: &HashMap<String, TagSchemas>,
) -> Result<(), String> {
    let mut client_path = out_path.to_path_buf();
    client_path.push("client.rs");

    // Sort tags alphabetically for deterministic output
    let mut sorted_tags: Vec<_> = tag_schemas.keys().collect();
    sorted_tags.sort();

    // Build a map of tag names to their deprecation notices
    let mut tag_deprecations: HashMap<String, String> = HashMap::new();
    for tag in &spec.tags {
        if let Some(serde_json::Value::String(notice)) = tag.extensions.get("x-deprecation-notice")
        {
            tag_deprecations.insert(tag.name.clone(), notice.clone());
        }
    }

    // Generate accessor methods for each tag client
    let mut tag_methods = Vec::new();
    for tag in sorted_tags {
        let snake_tag = tag.to_snake_case();
        let method_name = Ident::new(&snake_tag, Span::call_site());
        let client_module = Ident::new(&snake_tag, Span::call_site());
        let client_type = Ident::new(
            &format!("{}Client", tag.to_upper_camel_case()),
            Span::call_site(),
        );

        let deprecation_attr = if let Some(notice) = tag_deprecations.get(tag.as_str()) {
            quote! {
                #[deprecated = #notice]
            }
        } else {
            quote! {}
        };

        tag_methods.push(quote! {
            #deprecation_attr
            pub fn #method_name(&self) -> crate::resources::#client_module::#client_type<'_> {
                crate::resources::#client_module::#client_type::new(self)
            }
        });
    }

    let tokens = quote! {
        /// The main SumUp API client.
        ///
        /// Use this client to access different API endpoints organized by tags.
        #[derive(Debug, Clone)]
        pub struct Client {
            http_client: reqwest::Client,
            base_url: String,
            authorization_token: Option<String>,
            timeout: std::time::Duration,
        }

        impl Client {
            /// Creates a new SumUp API client with the default base URL.
            /// Tries to read the authorization token from the SUMUP_API_KEY environment variable.
            /// Default timeout is 10 seconds.
            pub fn new() -> Self {
                let authorization_token = std::env::var("SUMUP_API_KEY").ok();
                Self {
                    http_client: reqwest::Client::new(),
                    base_url: "https://api.sumup.com".to_string(),
                    authorization_token,
                    timeout: std::time::Duration::from_secs(10),
                }
            }

            /// Overrides the underlying HTTP client used for requests.
            /// Returns a new client with the provided `reqwest::Client`.
            pub fn with_client(mut self, http_client: reqwest::Client) -> Self {
                self.http_client = http_client;
                self
            }

            /// Sets the base URL for API requests.
            /// Returns a new client with the updated base URL.
            pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
                self.base_url = base_url.into();
                self
            }

            /// Sets the authorization token for API requests.
            /// Returns a new client with the updated token.
            pub fn with_authorization(mut self, token: impl Into<String>) -> Self {
                self.authorization_token = Some(token.into());
                self
            }

            /// Sets the request timeout for API requests.
            /// Returns a new client with the updated timeout.
            pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
                self.timeout = timeout;
                self
            }

            /// Returns a reference to the HTTP client.
            pub(crate) fn http_client(&self) -> &reqwest::Client {
                &self.http_client
            }

            /// Returns the base URL for the API.
            pub fn base_url(&self) -> &str {
                &self.base_url
            }

            /// Returns the authorization token if set.
            pub fn authorization_token(&self) -> Option<&str> {
                self.authorization_token.as_deref()
            }

            /// Returns the request timeout.
            pub fn timeout(&self) -> std::time::Duration {
                self.timeout
            }

            #(#tag_methods)*
        }

        impl Default for Client {
            fn default() -> Self {
                Self::new()
            }
        }
    };

    let contents = crate::format_generated_code(tokens);
    std::fs::write(&client_path, &contents)
        .map_err(|e| format!("Failed to write client.rs: {}", e))?;

    Ok(())
}
