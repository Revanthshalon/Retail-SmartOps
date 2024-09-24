use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

/// #### Health check handler.
///
/// This asynchronous function returns a JSON response indicating the health status of the application.
/// It responds with a status code of 200 (OK) and a JSON message.
///
/// ### Returns
///
/// A `Response` containing the status code and a JSON message.
pub async fn health_check() -> Response {
    let status_code = StatusCode::OK;
    let response = Json(json!({
        "message": "App is running"
    }));
    (status_code, response).into_response()
}
