use crate::{
    domain::entities::company::Company, infrastructure::entities::company::Model as CompanyModel,
};

impl From<CompanyModel> for Company {
    fn from(value: CompanyModel) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
            user_id: value.user_id,
        }
    }
}
