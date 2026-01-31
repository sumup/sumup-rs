#[cfg(feature = "tracing")]
mod imp {
    use tracing::{field, Span};

    pub struct RequestSpan {
        span: Span,
    }

    pub struct RequestSpanGuard<'a> {
        _guard: tracing::span::Entered<'a>,
    }

    impl RequestSpan {
        pub fn new(method: &'static str, path: &str, url: &str) -> Self {
            let span = tracing::info_span!(
                "sumup.request",
                "otel.kind" = "client",
                "otel.name" = "sumup.request",
                "http.request.method" = method,
                "url.full" = url,
                "url.path" = path,
                "http.response.status_code" = field::Empty,
                "error.message" = field::Empty,
                "otel.status_code" = field::Empty,
            );
            Self { span }
        }

        pub fn enter(&self) -> RequestSpanGuard<'_> {
            RequestSpanGuard {
                _guard: self.span.enter(),
            }
        }

        pub fn record_status(&self, status: reqwest::StatusCode) {
            self.span
                .record("http.response.status_code", status.as_u16());
        }

        pub fn record_error(&self, error: &reqwest::Error) {
            self.span.record("error.message", field::display(error));
            self.span.record("otel.status_code", "ERROR");
        }
    }
}

#[cfg(not(feature = "tracing"))]
mod imp {
    use std::marker::PhantomData;

    pub struct RequestSpan;

    pub struct RequestSpanGuard<'a> {
        _marker: PhantomData<&'a ()>,
    }

    impl RequestSpan {
        pub fn new(_method: &'static str, _path: &str, _url: &str) -> Self {
            Self
        }

        pub fn enter(&self) -> RequestSpanGuard<'_> {
            RequestSpanGuard {
                _marker: PhantomData,
            }
        }

        pub fn record_status(&self, _status: reqwest::StatusCode) {}

        pub fn record_error(&self, _error: &reqwest::Error) {}
    }
}

pub use imp::{RequestSpan, RequestSpanGuard};
