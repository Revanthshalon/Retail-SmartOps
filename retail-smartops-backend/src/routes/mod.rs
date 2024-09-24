use crate::AppState;
use axum::extract::{MatchedPath, Request};
use axum::response::Response;
use axum::Router;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};

mod health;

/// Creates the application routes and sets up tracing for HTTP requests.
///
/// The function configures a `ServiceBuilder` with a `TraceLayer` to handle tracing for HTTP requests.
/// It creates spans for each request, logs the start of the request, the response generation time,
/// and other tracing events.
///
/// # Returns
///
/// A `Router` instance with the configured routes and tracing layer.
pub fn create_app_routes(app_state: AppState) -> Router {
    let services = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Create a span for the HTTP request with the method and matched path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);
                    info_span!("http_request", method= ?request.method(), matched_path)
                })
                .on_request(|_request: &Request<_>, _span: &Span| tracing::info!("request started"))
                .on_response(|_response: &Response<_>, latency: Duration, _span: &Span| {
                    tracing::debug!("response generated in {:?}", latency)
                })
                .on_body_chunk(())
                .on_eos(())
                .on_failure(()),
        )
        .into_inner();

    Router::new()
        .nest("/api", health::create_health_routes(app_state.clone()))
        .layer(services)
}
