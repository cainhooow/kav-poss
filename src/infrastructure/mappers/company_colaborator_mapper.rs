use crate::{
    domain::entities::company_colaborator::CompanyColaborator,
    infrastructure::entities::company_colaborator::Model as CompanyColaboratorModel,
};

impl From<CompanyColaboratorModel> for CompanyColaborator {
    fn from(value: CompanyColaboratorModel) -> Self {
        Self {
            id: Some(value.id),
            company_id: value.company_id,
            user_id: value.user_id,
            document: value.document,
            badge: value.badge,
        }
    }
}
