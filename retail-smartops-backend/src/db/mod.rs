use crate::config::AppConfig;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

/// Service for managing database connections.
///
/// This struct holds a connection pool to the MySQL database.
pub struct DbService {
    pool: PgPool,
}

impl DbService {
    /// Creates a new instance of `DbService`.
    ///
    /// This asynchronous function initializes a connection pool to the MySQL database
    /// using the provided application configuration.
    ///
    /// # Arguments
    ///
    /// * `app_config` - An instance of `AppConfig` containing the database configuration values.
    ///
    /// # Returns
    ///
    /// A new instance of `DbService` with the initialized connection pool.
    ///
    /// # Panics
    ///
    /// This function will panic if the database connection pool cannot be created.
    pub async fn new(app_config: &AppConfig) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(app_config.get_max_connections())
            .idle_timeout(Duration::from_secs(app_config.get_idle_timeout()))
            .connect(&app_config.get_database_url())
            .await
            .expect("Failed to create database connection pool");
        Self { pool }
    }

    /// Retrieves the connection pool.
    ///
    /// This function returns a clone of the MySQL connection pool.
    ///
    /// # Returns
    ///
    /// A `MySqlPool` instance representing the connection pool.
    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }
}
