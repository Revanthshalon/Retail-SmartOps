use crate::repositories::RepositoryContainer;
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
