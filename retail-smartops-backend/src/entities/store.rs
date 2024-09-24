use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a store in the system.
///
/// This struct is used to store `store` information such as ID, owner ID, name, address,
/// and timestamps for when the store was created and last updated.
/// It derives `Debug`, `Serialize`, and `Deserialize` for easy debugging, serialization,
/// and deserialization.
#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    /// The unique identifier of the store.
    pub id: i32,
    /// The unique identifier of the owner of the store.
    pub owner_id: Uuid,
    /// The name of the store.
    pub name: String,
    /// The address of the store.
    pub address: Address,
    /// The timestamp when the store was created.
    pub created_at: DateTime<Utc>,
    /// The timestamp when the store was last updated.
    pub updated_at: DateTime<Utc>,
}

/// Represents an address in the system.
///
/// This struct is used to store address information such as country, state, city, street,
/// and zip code. It derives `Debug`, `Serialize`, `Deserialize`, and `sqlx::Type` for easy
/// debugging, serialization, deserialization, and database type mapping.
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "store_address")]
pub struct Address {
    /// The country of the address.
    pub country: String,
    /// The state of the address.
    pub state: String,
    /// The city of the address.
    pub city: String,
    /// The street of the address.
    pub street: String,
    /// The zip code of the address.
    pub zip: String,
}
