#[cfg(test)]
mod create_user_service_test {
    use std::rc::Rc;

    use self::create_user_service::CreateUserService;
    use super::super::*;
    use crate::{
        modules::user::{
            dtos::{create_user_serive_dto::CreateUserServiceDto, user_entity_dto::UserEntityDto},
            entities::user::User,
            repositories::users_repository::UsersRespository,
        },
        tests::repositories::users::users_in_memory_repository::UsersInMemoryRepository,
    };

    fn setup() -> (CreateUserService, Rc<dyn UsersRespository>) {
        let users_in_memory_repository: Rc<dyn UsersRespository> =
            Rc::new(UsersInMemoryRepository::new());
        let create_user_service = CreateUserService::new(Rc::clone(&users_in_memory_repository));

        return (create_user_service, users_in_memory_repository);
    }

    #[test]
    fn should_create_user() {
        let (sut, users_in_memory_repository) = setup();

        let create_user_serive_dto: CreateUserServiceDto = CreateUserServiceDto {
            name: "Jonas".to_string(),
            email: "jonas@jonas.com".to_string(),
            username: None,
        };

        let result = sut.execute(create_user_serive_dto);
        let user = users_in_memory_repository
            .find_by_email("jonas@jonas.com")
            .unwrap()
            .unwrap();

        assert!(result.is_ok(), "Create user failed");
        assert_eq!(
            "Jonas".to_string(),
            user.username(),
            "Has error on createion of user"
        );
        assert_eq!(
            "jonas@jonas.com".to_string(),
            user.email(),
            "Has error on creation of user"
        );
        assert_eq!(
            1,
            users_in_memory_repository.count().unwrap(),
            "Some error occured in users repository"
        );
    }

    #[test]
    fn should_fail_if_user_already_exists() {
        let (sut, users_in_memory_repository) = setup();

        let new_user_props = UserEntityDto {
            email: "jonas@jonas.com".to_string(),
            name: "Jonas".to_string(),
            username: None,
            id: None,
        };

        let user = User::new(new_user_props);
        let _r = users_in_memory_repository.create(&user);

        let create_user_serive_dto: CreateUserServiceDto = CreateUserServiceDto {
            name: "Jonas".to_string(),
            email: "jonas@jonas.com".to_string(),
            username: None,
        };

        let result = sut.execute(create_user_serive_dto);

        assert!(result.is_err(), "Result is not an error");
        assert_eq!(
            1,
            users_in_memory_repository.count().unwrap(),
            "Some error occured in users repository"
        );
    }
}
