use diesel::{prelude::*, Insertable, Queryable};
use serde::Deserialize;

#[derive(Queryable, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserDiesel {
    pub id: String,
    pub name: String,
    pub username: Option<String>,
    pub password: String,
    pub email: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUserDiesel<'a> {
    id: &'a str,
    name: &'a str,
    username: Option<&'a str>,
    password: &'a str,
    email: &'a str,
}
