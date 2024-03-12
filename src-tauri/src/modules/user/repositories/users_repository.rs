use crate::modules::user::entities::user::User;
use crate::shared::core::errors::repository_error::RepositoryError;

pub trait UsersRepository {
    fn create(&self, user: &User) -> Result<(), RepositoryError>;
    fn find_by_id(&self, id: &str) -> Result<Option<User>, RepositoryError>;
    fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError>;
    fn count(&self) -> Result<usize, RepositoryError>;
}
