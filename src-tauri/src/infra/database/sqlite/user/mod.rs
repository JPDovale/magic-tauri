use diesel::SqliteConnection;

use crate::{
    modules::user::{entities::user::User, repositories::users_repository::UsersRepository},
    shared::core::errors::repository_error::RepositoryError,
};
mod models;
struct UsersDieselRepository {
    conn: SqliteConnection,
}

impl UsersDieselRepository {
    pub fn new(conn: SqliteConnection) -> Self {
        UsersDieselRepository { conn }
    }
}

impl UsersRepository for UsersDieselRepository {
    fn create(&self, user: &User) -> Result<(), RepositoryError> {
        diesel::insert_into(users)
    }

    fn find_by_id(&self, id: &str) -> Result<Option<User>, RepositoryError> {
        todo!()
    }

    fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        todo!()
    }

    fn count(&self) -> Result<usize, RepositoryError> {
        todo!()
    }
}
