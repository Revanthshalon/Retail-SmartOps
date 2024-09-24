use crate::models::user::CreateUserDTO;
use crate::repositories::RepositoryContainer;
use axum::response::Response;
use std::sync::Arc;

pub struct UserAccessManagementService {
    repository_container: Arc<RepositoryContainer>,
}

impl UserAccessManagementService {
    pub fn new(repository_container: Arc<RepositoryContainer>) -> Self {
        Self {
            repository_container,
        }
    }
}

impl UserAccessManagementService {
    pub async fn register_user(&self, payload: CreateUserDTO) -> Response {}
}
