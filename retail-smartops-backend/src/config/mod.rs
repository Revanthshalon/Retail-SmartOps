/// Configuration for the application.
///
/// This struct holds the configuration values for the database and server.
/// The values are loaded from environment variables.
pub struct AppConfig {
    database_username: String,
    database_password: String,
    database_host: String,
    database_port: u16,
    database_name: String,
    database_max_connections: u32,
    database_idle_timeout: u64,
    server_host: String,
    server_port: u16,
}

impl AppConfig {
    /// Loads the application configuration from environment variables.
    ///
    /// This function uses the `dotenvy` crate to load environment variables from a `.env` file,
    /// and then reads the necessary configuration values from the environment.
    ///
    /// # Panics
    ///
    /// This function will panic if any of the required environment variables are not set or
    /// if they cannot be parsed into the expected types.
    ///
    /// # Returns
    ///
    /// An instance of `AppConfig` with the loaded configuration values.
    pub fn load() -> Self {
        use dotenvy::dotenv;
        use std::env;

        dotenv().ok();

        let database_username =
            env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set");
        let database_password =
            env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
        let database_host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
        let database_port = env::var("DATABASE_PORT")
            .expect("DATABASE_PORT must be set")
            .parse::<u16>()
            .expect("DATABASE_PORT must be a valid number");
        let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
        let database_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .expect("DATABASE_MAX_CONNECTIONS must be set")
            .parse::<u32>()
            .expect("DATABASE_MAX_CONNECTIONS must be a valid number");
        let database_idle_timeout = env::var("DATABASE_IDLE_TIMEOUT")
            .expect("DATABASE_IDLE_TIMEOUT must be set")
            .parse::<u64>()
            .expect("DATABASE_IDLE_TIMEOUT must be a valid number");
        let server_host = env::var("SERVER_HOST").expect("SERVER_HOST must be set");
        let server_port = env::var("SERVER_PORT")
            .expect("SERVER_PORT must be set")
            .parse::<u16>()
            .expect("SERVER_PORT must be a valid number");

        Self {
            database_username,
            database_password,
            database_host,
            database_port,
            database_name,
            database_max_connections,
            database_idle_timeout,
            server_host,
            server_port,
        }
    }

    /// Constructs the database connection URL.
    ///
    /// This function creates a connection URL for the PostgreSQL database using the loaded
    /// configuration values.
    ///
    /// # Returns
    ///
    /// A `String` containing the database connection URL.
    pub fn get_database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database_username,
            self.database_password,
            self.database_host,
            self.database_port,
            self.database_name
        )
    }

    /// Constructs the server address.
    ///
    /// This function creates the server address using the loaded configuration values.
    ///
    /// # Returns
    ///
    /// A `String` containing the server address.
    pub fn get_server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    /// Gets the maximum number of database connections.
    ///
    /// # Returns
    ///
    /// A `u32` representing the maximum number of database connections.
    pub fn get_max_connections(&self) -> u32 {
        self.database_max_connections
    }

    /// Gets the idle timeout for database connections.
    ///
    /// # Returns
    ///
    /// A `u64` representing the idle timeout for database connections in seconds.
    pub fn get_idle_timeout(&self) -> u64 {
        self.database_idle_timeout
    }
}
