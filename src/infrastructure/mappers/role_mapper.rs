use std::str::FromStr;

use core_server::RoleEnum;

use crate::domain::entities::role::Role;
use crate::infrastructure::entities::role::Model as RoleModel;

impl From<RoleModel> for RoleEnum {
    fn from(value: RoleModel) -> Self {
        RoleEnum::from_str(&value.name).unwrap()
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
