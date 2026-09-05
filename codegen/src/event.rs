use heck::{ToSnakeCase, ToUpperCamelCase};
use oas3::Spec as OpenAPI;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[derive(Debug)]
struct EventDefinition {
    tag: String,
    event_type: String,
    marker_ident: Ident,
    event_alias_ident: Ident,
    variant_ident: Ident,
    object_type_ident: Ident,
    object_module_ident: Ident,
}

/// Generates the event catalog derived from the OpenAPI top-level `webhooks` map.
pub fn generate_events_file(out_path: &std::path::Path, spec: &OpenAPI) -> Result<(), String> {
    let definitions = collect_event_definitions(spec)?;
    let tokens = generate_events_tokens(&definitions);
    let contents = crate::format_generated_code(tokens);

    let mut events_path = out_path.to_path_buf();
    events_path.push("events.rs");

    std::fs::write(&events_path, contents).map_err(|e| format!("Failed to write events.rs: {}", e))
}

/// Generates event marker types and aliases that belong to a resource tag module.
pub fn generate_tag_event_tokens(spec: &OpenAPI, tag: &str) -> Result<TokenStream, String> {
    let definitions = collect_event_definitions(spec)?;
    let tag_definitions: Vec<_> = definitions
        .iter()
        .filter(|definition| definition.tag == tag)
        .collect();

    let marker_defs = tag_definitions.iter().map(|definition| {
        let marker_ident = &definition.marker_ident;
        let event_alias_ident = &definition.event_alias_ident;
        let object_type_ident = &definition.object_type_ident;
        let event_type = &definition.event_type;

        quote! {
            /// Marker type for this event notification.
            #[derive(Debug, Clone)]
            pub enum #marker_ident {}

            impl crate::event::private::Sealed for #marker_ident {}

            impl crate::events::EventSpec for #marker_ident {
                const EVENT_TYPE: &'static str = #event_type;

                type FetchedObject = #object_type_ident;
            }

            /// Event notification for this event type.
            pub type #event_alias_ident = crate::events::Event<#marker_ident>;
        }
    });

    Ok(quote! {
        #(#marker_defs)*
    })
}

fn collect_event_definitions(spec: &OpenAPI) -> Result<Vec<EventDefinition>, String> {
    let mut definitions = Vec::new();

    for (event_type, path_item) in &spec.webhooks {
        let Some(operation) = path_item.post.as_ref() else {
            continue;
        };

        let tag = operation
            .tags
            .first()
            .ok_or_else(|| format!("Event '{}' is missing a tag", event_type))?;
        let object_reference = operation
            .extensions
            .get("object")
            .and_then(|object| object.get("$ref"))
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| format!("Event '{}' is missing x-object", event_type))?;
        let object_schema = object_reference
            .strip_prefix("#/components/schemas/")
            .ok_or_else(|| {
                format!(
                    "Event '{}' has unsupported x-object reference '{}'",
                    event_type, object_reference
                )
            })?;
        let operation_id = operation
            .operation_id
            .as_deref()
            .ok_or_else(|| format!("Event '{}' is missing operationId", event_type))?;
        let marker_name = operation_id
            .strip_suffix("Webhook")
            .unwrap_or(operation_id)
            .to_upper_camel_case();
        let event_alias_name = format!("{marker_name}Event");

        definitions.push(EventDefinition {
            tag: tag.to_string(),
            event_type: event_type.clone(),
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
    let registration_methods = definitions.iter().map(|definition| {
        let method_ident = Ident::new(
            &format!("on_{}", definition.marker_ident.to_string().to_snake_case()),
            Span::call_site(),
        );
        let marker_ident = &definition.marker_ident;
        let event_alias_ident = &definition.event_alias_ident;
        let object_module_ident = &definition.object_module_ident;
        let doc = format!(" Registers an async callback for `{}` notifications.", definition.event_type);

        quote! {
            #[doc = #doc]
            ///
            /// The callback receives the typed event and a clone of the client.
            /// Returns an error if a callback is already registered for this event.
            pub fn #method_ident<HandlerFuture>(
                &mut self,
                callback: impl Fn(crate::resources::#object_module_ident::#event_alias_ident, crate::Client) -> HandlerFuture
                    + Send + Sync + 'static,
            ) -> Result<&mut Self, EventHandlerRegistrationError>
            where
                HandlerFuture: std::future::Future + Send + 'static,
                HandlerFuture::Output: IntoEventHandlerResult + 'static,
            {
                self.register::<crate::resources::#object_module_ident::#marker_ident, _>(callback)
            }
        }
    });

    let variants = definitions.iter().map(|definition| {
        let variant_ident = &definition.variant_ident;
        let event_alias_ident = &definition.event_alias_ident;
        let object_module_ident = &definition.object_module_ident;
        quote! {
            #variant_ident(crate::resources::#object_module_ident::#event_alias_ident)
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
            <crate::resources::#object_module_ident::#marker_ident as EventSpec>::EVENT_TYPE => EventNotification::#variant_ident(
                crate::resources::#object_module_ident::#event_alias_ident::from_raw(client, event),
            ),
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
        //! Event receivers should read the HTTP request body as raw bytes. Most
        //! integrations can register typed async callbacks with the generated
        //! `on_*` methods on [`EventsHandler`] and pass the body together with the
        //! `X-SumUp-Webhook-Signature` header to
        //! [`EventsHandler::handle`]. Use [`crate::Client::parse_event_notification`]
        //! when direct matching is a better fit. Both paths verify the signature
        //! and enforces a fixed five-minute clock skew before dispatching or returning an event.

        pub use crate::event::{
            verify_signature, Event, EventCallbackError, EventError,
            EventHandlerRegistrationError, EventHandlingError, EventObject, EventSpec,
            EventsHandler, FetchObject, IntoEventHandlerResult, UnknownEvent,
            SIGNATURE_HEADER, SIGNATURE_VERSION,
        };
        pub(crate) use crate::event::RawEvent;

        impl EventsHandler {
            #(#registration_methods)*
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
            #(#variants,)*
            Unknown(UnknownEvent),
        }

        impl EventNotification {
            /// Returns the event type string, such as `members.updated`.
            pub fn event_type(&self) -> &str {
                match self {
                    #(#event_type_arms)*
                    Self::Unknown(event) => &event.event_type,
                }
            }
        }

        pub(crate) fn parse_known_event(
            client: crate::Client,
            event: RawEvent,
        ) -> EventNotification {
            match event.event_type() {
                #(#parse_arms)*
                _ => EventNotification::Unknown(UnknownEvent::from_raw(
                    client, event,
                )),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn collects_events_from_typed_spec_extensions() {
        let spec: OpenAPI = serde_json::from_value(json!({
            "openapi": "3.1.0",
            "info": {
                "title": "Test API",
                "version": "1.0.0"
            },
            "webhooks": {
                "members.updated": {
                    "post": {
                        "operationId": "MemberUpdatedWebhook",
                        "tags": ["Members"],
                        "x-object": {
                            "$ref": "#/components/schemas/Member"
                        }
                    }
                }
            }
        }))
        .expect("parse spec");

        let definitions = collect_event_definitions(&spec).expect("collect events");

        assert_eq!(definitions.len(), 1);
        assert_eq!(definitions[0].event_type, "members.updated");
        assert_eq!(definitions[0].tag, "Members");
        assert_eq!(definitions[0].marker_ident, "MemberUpdated");
        assert_eq!(definitions[0].object_type_ident, "Member");
        assert_eq!(definitions[0].object_module_ident, "members");
    }
}
