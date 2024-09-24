use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a user role in the system.
///
/// This struct is used to map the relationship between users and their roles.
/// It derives `Debug`, `Serialize`, `Deserialize`, and `sqlx::FromRow` for easy
/// debugging, serialization, deserialization, and database row mapping.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserRole {
    /// The unique identifier of the user.
    pub user_id: Uuid,
    /// The identifier of the role assigned to the user.
    pub role_id: i32,
    /// The timestamp when the role was assigned to the user.
    pub assigned_at: DateTime<Utc>,
}
