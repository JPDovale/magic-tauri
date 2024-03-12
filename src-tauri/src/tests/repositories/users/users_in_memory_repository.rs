use std::{borrow::Borrow, cell::RefCell};

use crate::{
    modules::user::{entities::user::User, repositories::users_repository::UsersRepository},
    shared::core::errors::repository_error::RepositoryError,
};

#[derive(Debug, Clone)]
pub struct UsersInMemoryRepository {
    pub users: RefCell<Vec<User>>,
}

impl UsersInMemoryRepository {
    pub fn new() -> Self {
        UsersInMemoryRepository {
            users: RefCell::new(Vec::new()),
        }
    }
}

impl UsersRepository for UsersInMemoryRepository {
    fn create(&self, user: &User) -> Result<(), RepositoryError> {
        self.users.borrow_mut().push(user.clone());
        Ok(())
    }

    fn find_by_id(&self, id: &str) -> Result<Option<User>, RepositoryError> {
        let users_ref = self.users.borrow();
        let users = users_ref.iter();

        for user in users {
            if user.id().to_string().eq(id) {
                return Ok(Some(user.clone()));
            }
        }

        Ok(None)
    }

    fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        let users_ref = self.users.borrow();
        let users = users_ref.iter();

        for user in users {
            if user.email().eq(email) {
                return Ok(Some(user.clone()));
            }
        }

        Ok(None)
    }

    fn count(&self) -> Result<usize, RepositoryError> {
        let size = self.users.borrow().len();
        Ok(size)
    }
}
