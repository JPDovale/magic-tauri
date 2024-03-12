use crate::modules::user::dtos::user_entity_dto::UserEntityDto;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    username: String,
    email: String,
    name: String,
}

impl User {
    pub fn new(params: UserEntityDto) -> User {
        User {
            id: params.id.unwrap_or_else(Uuid::new_v4),
            username: params.username.unwrap_or_else(|| params.name.clone()),
            email: params.email,
            name: params.name,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> String {
        self.email.to_string()
    }

    pub fn username(&self) -> String {
        self.username.to_string()
    }
}
