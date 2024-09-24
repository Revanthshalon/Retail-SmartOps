use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct UserRoleResponseDTO {
    pub user_id: Uuid,
    pub role_id: i32,
}
