use crate::{
    domain::entities::company_colaborator::CompanyColaborator,
    domain::entities::company_role::CompanyRole,
    infrastructure::entities::company_colaborator::Model as CompanyColaboratorModel,
    infrastructure::entities::company_role::Model as CompanyRoleModel,
};

impl From<CompanyColaboratorModel> for CompanyColaborator {
    fn from(value: CompanyColaboratorModel) -> Self {
        Self {
            id: Some(value.id),
            company_id: value.company_id,
            user_id: value.user_id,
            document: value.document,
            badge: value.badge,
            roles: vec![],
        }
    }
}

impl From<(CompanyColaboratorModel, Vec<CompanyRoleModel>)> for CompanyColaborator {
    fn from(value: (CompanyColaboratorModel, Vec<CompanyRoleModel>)) -> Self {
        Self {
            id: Some(value.0.id),
            company_id: value.0.company_id,
            user_id: value.0.user_id,
            document: value.0.document,
            badge: value.0.badge,
            roles: value.1.into_iter().map(CompanyRole::from).collect(),
        }
    }
}
