use crate::repositories::RepositoryContainer;
use crate::services::user_access_management_service::UserAccessManagementService;
use std::sync::Arc;

mod user_access_management_service;

pub struct ServiceContainer {
    pub user_access_management_service: UserAccessManagementService,
}

impl ServiceContainer {
    pub fn new(repository_container: Arc<RepositoryContainer>) -> Self {
        Self {
            user_access_management_service: UserAccessManagementService::new(
                repository_container.clone(),
            ),
        }
    }
}
