use crate::models::user::{CreateUserDTO, UpdateUserDTO};
use crate::repositories::RepositoryContainer;
use axum::response::Response;
use std::sync::Arc;
use uuid::Uuid;

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
    pub async fn register_user(&self, payload: CreateUserDTO) -> Response {
        todo!()
    }

    pub async fn login_user(&self, username: &str, password: &str) -> Response {
        todo!()
    }

    pub async fn logout_user(&self, username: &str) -> Response {
        todo!()
    }

    pub async fn get_user(&self, username: &str) -> Response {
        todo!()
    }

    pub async fn get_employees(&self) -> Response {
        todo!()
    }

    pub async fn get_employee_by_id(&self, employee_id: Uuid) -> Response {
        todo!()
    }

    pub async fn update_employee(&self, employee_id: Uuid, payload: UpdateUserDTO) -> Response {
        todo!()
    }

    pub async fn deactivate_employee(&self, employee_id: Uuid) -> Response {
        todo!()
    }

    pub async fn reactive_employee(&self, employee_id: Uuid) -> Response {
        todo!()
    }
}
