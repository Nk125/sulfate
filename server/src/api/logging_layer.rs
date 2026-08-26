use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
};
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tracing::{Span, info_span};

const CHUNK_BYTES_SHOWN: usize = 5;

pub fn span<T>(request: &Request<T>) -> Span {
    let matched_path = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);

    info_span!(
        "http_request",
        version = ?request.version(),
        method = ?request.method(),
        matched_path,
        // Real path
        uri = %request.uri()
    )
}

pub fn request<T>(request: &Request<T>, _span: &Span) {
    let headers = request.headers();

    tracing::debug!(
        user_agent = headers
            .get(axum::http::header::USER_AGENT)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("-"),
        size_hint = headers
            .get(axum::http::header::CONTENT_LENGTH)
            .and_then(|len| len.to_str().ok())
            .unwrap_or("-"),
        "request received"
    );
}

pub fn response(response: &Response, latency: Duration, _span: &Span) {
    tracing::info!(
        code = %response.status(),
        size_hint = response.headers()
            .get(axum::http::header::CONTENT_LENGTH)
            .and_then(|len| len.to_str().ok())
            .unwrap_or("-"),
        "took {}ms",
        latency.as_millis()
    );
}

pub fn body_chunk(chunk: &Bytes, latency: Duration, _span: &Span) {
    let first_bytes = chunk.first_chunk::<CHUNK_BYTES_SHOWN>();

    tracing::trace!(
        ?first_bytes,
        chunk_size = chunk.len(),
        "chunk transferred in {}ms",
        latency.as_millis()
    );
}

pub fn eos(trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span) {
    tracing::debug!(
        has_trailers = trailers.is_some(),
        "request finished in {}ms",
        stream_duration.as_millis()
    );
}

pub fn failure(error: ServerErrorsFailureClass, latency: Duration, _span: &Span) {
    tracing::error!(?error, "request failed, took {}ms", latency.as_millis());
}
