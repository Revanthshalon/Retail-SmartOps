use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a user in the system.
///
/// This struct is used to store user information such as ID, username, password,
/// and timestamps for when the user was created and last updated.
/// It derives `Debug`, `Serialize`, `Deserialize`, and `sqlx::FromRow` for easy
/// debugging, serialization, deserialization, and database row mapping.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    /// The unique identifier of the user.
    pub id: Uuid,
    /// The username of the user.
    pub username: String,
    /// The password of the user.
    pub password: String,
    /// The email of the user.
    pub email: String,
    /// The timestamp when the user was created.
    pub created_at: DateTime<Utc>,
    /// The timestamp when the user was last updated.
    pub updated_at: DateTime<Utc>,
    /// Indicates if the user is active.
    pub is_active: bool,
}
