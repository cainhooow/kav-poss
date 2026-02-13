use crate::{
    domain::entities::company_role::CompanyRole,
    infrastructure::entities::company_role::Model as CompanyRoleModel,
};

impl From<CompanyRoleModel> for CompanyRole {
    fn from(value: CompanyRoleModel) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
            description: value.description,
            company_id: value.company_id,
        }
    }
}
