use crate::{
    domain::entities::company_role::CompanyRole, domain::entities::role::Role,
    infrastructure::entities::company_role::Model as CompanyRoleModel,
    infrastructure::entities::company_role::ModelEx as CompanyRoleModelEx,
    infrastructure::entities::role::Model as RoleModel,
};

impl From<CompanyRoleModel> for CompanyRole {
    fn from(value: CompanyRoleModel) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
            description: value.description,
            company_id: value.company_id,
            flags: vec![],
        }
    }
}

impl From<CompanyRoleModelEx> for CompanyRole {
    fn from(value: CompanyRoleModelEx) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
            description: value.description,
            company_id: value.company_id,
            flags: value.flags.into_iter().map(Role::from).collect(),
        }
    }
}

impl From<(CompanyRoleModel, Vec<RoleModel>)> for CompanyRole {
    fn from(value: (CompanyRoleModel, Vec<RoleModel>)) -> Self {
        Self {
            id: Some(value.0.id),
            name: value.0.name,
            description: value.0.description,
            company_id: value.0.company_id,
            flags: value.1.into_iter().map(Role::from).collect::<Vec<Role>>(),
        }
    }
}
