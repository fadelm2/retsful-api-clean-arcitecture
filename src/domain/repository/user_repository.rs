use super::super::entity::user_entity::User;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &User) -> Result<User, String>;
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, String>;
    async fn find_user_by_id(&self, id: &Uuid) -> Result<Option<User>, String>;
}
