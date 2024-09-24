use crate::config::AppConfig;
use crate::db::DbService;
use crate::repositories::RepositoryContainer;
use crate::routes::create_app_routes;
use crate::services::ServiceContainer;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod config;
mod db;
mod entities;
mod errors;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

/// Runs the application by setting up tracing, creating application routes, and starting the server.
///
/// #### Arguments
///
/// * `listener` - An optional `TcpListener` to bind the server to a specific address and port.
///
/// #### Returns
///
/// A `Result` which is `Ok` if the server runs successfully, or an `std::io::Error` if an error occurs.
pub async fn run_app(listener: Option<TcpListener>) -> Result<(), std::io::Error> {
    // Initialize the tracing subscriber with environment filter and formatting layer.
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!(
                "{}=debug,tower_http=debug,axum::rejection=trace",
                env!("CARGO_CRATE_NAME")
            )
            .into()
        }))
        .with(fmt::layer())
        .init();

    let app_config = AppConfig::load();
    let db_service = DbService::new(&app_config).await;

    let repository_container = RepositoryContainer::new(db_service.get_pool());

    let app_state = AppState::new(app_config, repository_container);

    let _service_container = ServiceContainer::new(app_state.repository_container.clone());

    // Create application routes.
    let app_routes = create_app_routes(app_state.clone());

    // Bind the listener to the specified address and port, or default to "0.0.0.0:3000".
    let listener = match listener {
        Some(listener) => listener,
        None => TcpListener::bind(app_state.app_config.get_server_address())
            .await
            .expect("Failed to bind port"),
    };

    // Serve the application using the specified listener and routes.
    axum::serve(listener, app_routes.into_make_service()).await
}

#[derive(Clone)]
pub struct AppState {
    app_config: Arc<AppConfig>,
    repository_container: Arc<RepositoryContainer>,
}

impl AppState {
    pub fn new(app_config: AppConfig, repository_container: RepositoryContainer) -> Self {
        Self {
            app_config: Arc::new(app_config),
            repository_container: Arc::new(repository_container),
        }
    }
}
