use crate::domain::entities::role::Role;
use crate::infrastructure::entities::role::Model as RoleModel;

impl From<RoleModel> for Role {
    fn from(value: RoleModel) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
            description: value.description,
        }
    }
}
