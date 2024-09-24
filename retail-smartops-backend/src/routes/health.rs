use crate::handlers::health::health_check;
use crate::AppState;
use axum::routing::get;
use axum::Router;

pub fn create_health_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(app_state)
}
