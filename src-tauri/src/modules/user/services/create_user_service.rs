use std::rc::Rc;

use crate::{
    modules::user::{
        dtos::{create_user_serive_dto::CreateUserServiceDto, user_entity_dto::UserEntityDto},
        entities::user::User,
        repositories::users_repository::UsersRespository,
    },
    shared::core::errors::service_error::ServiceError,
};

pub struct CreateUserServiceResponse {
    user: User,
}

pub struct CreateUserService {
    users_repository: Rc<dyn UsersRespository>,
}

impl CreateUserService {
    pub fn new(users_repository: Rc<dyn UsersRespository>) -> CreateUserService {
        CreateUserService { users_repository }
    }

    pub fn execute(
        &self,
        req: CreateUserServiceDto,
    ) -> Result<CreateUserServiceResponse, ServiceError> {
        let CreateUserServiceDto {
            email,
            name,
            username,
        } = req;

        let find_by_email_result = self.users_repository.find_by_email(&email);

        if let Ok(Some(_)) = find_by_email_result {
            return Err(ServiceError::user_already_exists());
        }

        if find_by_email_result.is_err() {
            return Err(ServiceError::internal_error());
        }

        let user_props: UserEntityDto = UserEntityDto {
            id: None,
            username,
            name,
            email,
        };

        let user: User = User::new(user_props);
        let create_user_result = self.users_repository.create(&user);

        if create_user_result.is_err() {
            return Err(ServiceError::internal_error());
        }

        Ok(CreateUserServiceResponse { user })
    }
}
