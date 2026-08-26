pub mod client;
pub mod consumer;
mod logging_layer;

use tower_http::trace::TraceLayer;

pub fn routes() -> axum::Router {
    use axum::routing::get;

    axum::Router::new()
        .nest("/consumer", consumer::routes())
        .nest("/client", client::routes())
        .route("/health", get(async || "sulfate"))
        .route("/version", get(async || env!("CARGO_PKG_VERSION")))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(logging_layer::span)
                .on_request(logging_layer::request)
                .on_response(logging_layer::response)
                .on_body_chunk(logging_layer::body_chunk)
                .on_eos(logging_layer::eos)
                .on_failure(logging_layer::failure),
        )
}
