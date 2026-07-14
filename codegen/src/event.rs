use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EventOperation {
    #[serde(rename = "operationId")]
    operation_id: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(rename = "x-object")]
    object: Option<EventObjectRef>,
    #[serde(rename = "x-object-type")]
    object_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EventObjectRef {
    #[serde(rename = "$ref")]
    reference: String,
}

#[derive(Debug, Deserialize)]
struct EventPathItem {
    post: Option<EventOperation>,
}

#[derive(Debug)]
struct EventDefinition {
    tag: String,
    event_type: String,
    object_type: String,
    marker_ident: Ident,
    event_alias_ident: Ident,
    variant_ident: Ident,
    object_type_ident: Ident,
    object_module_ident: Ident,
}

/// Generates the event catalog derived from the OpenAPI top-level `webhooks` map.
pub fn generate_events_file(
    out_path: &std::path::Path,
    raw_spec: &serde_json::Value,
) -> Result<(), String> {
    let definitions = collect_event_definitions(raw_spec)?;
    let tokens = generate_events_tokens(&definitions);
    let contents = crate::format_generated_code(tokens);

    let mut events_path = out_path.to_path_buf();
    events_path.push("events.rs");

    std::fs::write(&events_path, contents).map_err(|e| format!("Failed to write events.rs: {}", e))
}

/// Generates event marker types and aliases that belong to a resource tag module.
pub fn generate_tag_event_tokens(
    raw_spec: &serde_json::Value,
    tag: &str,
) -> Result<TokenStream, String> {
    let definitions = collect_event_definitions(raw_spec)?;
    let tag_definitions: Vec<_> = definitions
        .iter()
        .filter(|definition| definition.tag == tag)
        .collect();

    let marker_defs = tag_definitions.iter().map(|definition| {
        let marker_ident = &definition.marker_ident;
        let event_alias_ident = &definition.event_alias_ident;
        let object_type_ident = &definition.object_type_ident;
        let event_type = &definition.event_type;
        let object_type = &definition.object_type;

        quote! {
            /// Marker type for this event notification.
            #[derive(Debug, Clone)]
            pub enum #marker_ident {}

            impl crate::events::EventSpec for #marker_ident {
                const EVENT_TYPE: &'static str = #event_type;
                const OBJECT_TYPE: &'static str = #object_type;

                type FetchedObject = #object_type_ident;
            }

            /// Event notification for this event type.
            pub type #event_alias_ident<'a> = crate::events::Event<'a, #marker_ident>;
        }
    });

    Ok(quote! {
        #(#marker_defs)*
    })
}

fn collect_event_definitions(raw_spec: &serde_json::Value) -> Result<Vec<EventDefinition>, String> {
    let Some(webhooks_value) = raw_spec.get("webhooks") else {
        return Ok(Vec::new());
    };

    let webhooks: indexmap::IndexMap<String, EventPathItem> =
        serde_json::from_value(webhooks_value.clone())
            .map_err(|e| format!("Failed to parse OpenAPI webhooks: {}", e))?;

    let mut definitions = Vec::new();

    for (event_type, path_item) in webhooks {
        let Some(operation) = path_item.post else {
            continue;
        };

        let tag = operation
            .tags
            .first()
            .ok_or_else(|| format!("Event '{}' is missing a tag", event_type))?;
        let object = operation
            .object
            .ok_or_else(|| format!("Event '{}' is missing x-object", event_type))?;
        let object_schema = object
            .reference
            .strip_prefix("#/components/schemas/")
            .ok_or_else(|| {
                format!(
                    "Event '{}' has unsupported x-object reference '{}'",
                    event_type, object.reference
                )
            })?;
        let object_type = operation
            .object_type
            .ok_or_else(|| format!("Event '{}' is missing x-object-type", event_type))?;

        let marker_name = operation
            .operation_id
            .strip_suffix("Webhook")
            .unwrap_or(&operation.operation_id)
            .to_upper_camel_case();
        let event_alias_name = format!("{marker_name}Event");

        definitions.push(EventDefinition {
            tag: tag.to_string(),
            event_type,
            object_type,
            marker_ident: Ident::new(&marker_name, Span::call_site()),
            event_alias_ident: Ident::new(&event_alias_name, Span::call_site()),
            variant_ident: Ident::new(&marker_name, Span::call_site()),
            object_type_ident: Ident::new(&object_schema.to_upper_camel_case(), Span::call_site()),
            object_module_ident: Ident::new(&tag.to_snake_case(), Span::call_site()),
        });
    }

    definitions.sort_by(|a, b| {
        a.variant_ident
            .to_string()
            .cmp(&b.variant_ident.to_string())
    });
    Ok(definitions)
}

fn generate_events_tokens(definitions: &[EventDefinition]) -> TokenStream {
    let variants = definitions.iter().map(|definition| {
        let variant_ident = &definition.variant_ident;
        let event_alias_ident = &definition.event_alias_ident;
        let object_module_ident = &definition.object_module_ident;
        quote! {
            #variant_ident(crate::resources::#object_module_ident::#event_alias_ident<'a>)
        }
    });

    let event_type_arms = definitions.iter().map(|definition| {
        let variant_ident = &definition.variant_ident;
        quote! {
            Self::#variant_ident(event) => event.event_type(),
        }
    });

    let parse_arms = definitions.iter().map(|definition| {
        let marker_ident = &definition.marker_ident;
        let variant_ident = &definition.variant_ident;
        let event_alias_ident = &definition.event_alias_ident;
        let object_module_ident = &definition.object_module_ident;
        quote! {
            <crate::resources::#object_module_ident::#marker_ident as EventSpec>::EVENT_TYPE => Ok(EventNotification::#variant_ident(
                crate::resources::#object_module_ident::#event_alias_ident::from_raw(client, event),
            )),
        }
    });

    quote! {
        //! Verify and parse event notifications sent by SumUp.
        //!
        //! Events let your integration react to changes in SumUp without polling the
        //! API. Use them to update orders in your own system when a resource changes,
        //! trigger fulfillment or accounting workflows, reconcile asynchronous state,
        //! and keep local records in sync with SumUp.
        //!
        //! Event receivers should read the HTTP request body as raw bytes and pass it
        //! together with the `X-SumUp-Webhook-Signature` and
        //! `X-SumUp-Webhook-Timestamp` headers to [`EventsHandler::parse`]. The SDK
        //! verifies the signature and timestamp before deserializing the payload.

        pub use crate::event::{
            verify_signature, Event, EventError, EventFetchError, EventObject, EventSpec,
            EventsHandler, FetchObject, UnknownEvent, DEFAULT_TOLERANCE, SIGNATURE_HEADER,
            SIGNATURE_VERSION, TIMESTAMP_HEADER,
        };
        pub(crate) use crate::event::RawEvent;

        /// Event notification parsed by the SDK.
        ///
        /// Known event types are represented by dedicated variants. Unknown event types
        /// are preserved as [`EventNotification::Unknown`] so your integration can
        /// safely acknowledge or log them without losing the raw event type and object
        /// reference.
        #[derive(Debug, Clone)]
        #[non_exhaustive]
        pub enum EventNotification<'a> {
            #(#variants,)*
            Unknown(UnknownEvent<'a>),
        }

        impl EventNotification<'_> {
            /// Returns the event type string, such as `members.updated`.
            pub fn event_type(&self) -> &str {
                match self {
                    #(#event_type_arms)*
                    Self::Unknown(event) => &event.event_type,
                }
            }
        }

        pub(crate) fn parse_known_event<'a>(
            client: &'a crate::Client,
            event: RawEvent,
        ) -> Result<EventNotification<'a>, EventError> {
            match event.event_type() {
                #(#parse_arms)*
                _ => Ok(EventNotification::Unknown(UnknownEvent::from_raw(
                    client, event,
                ))),
            }
        }
    }
}
