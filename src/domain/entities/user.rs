use crate::domain::entities::role::RolesEnum;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<RolesEnum>
}

#[derive(Clone, Debug)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}