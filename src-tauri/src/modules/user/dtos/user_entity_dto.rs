use uuid::Uuid;

pub struct UserEntityDto {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: String,
    pub name: String,
}
