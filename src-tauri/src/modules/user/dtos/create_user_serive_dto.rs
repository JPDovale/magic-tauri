#[derive(Debug)]
pub struct CreateUserServiceDto {
    pub name: String,
    pub username: Option<String>,
    pub email: String,
}
