use crate::domain::entities::role::{Role, RolesEnum};
use crate::infrastructure::entities::role::Model as RoleModel;

impl From<RoleModel> for RolesEnum {
    fn from(value: RoleModel) -> Self {
        RolesEnum::from_str(&value.name).unwrap()
    }
}

impl From<RoleModel> for Role {
    fn from(value: RoleModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}
