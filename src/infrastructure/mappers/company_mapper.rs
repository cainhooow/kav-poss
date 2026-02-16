use crate::{
    domain::entities::{company::Company, company_colaborator::CompanyColaborator},
    infrastructure::entities::{
        company::Model as CompanyModel, company_colaborator::Model as ColaboratorModel,
    },
};

impl From<CompanyModel> for Company {
    fn from(value: CompanyModel) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
            user_id: value.user_id,
            colaborators: vec![],
        }
    }
}

impl From<(CompanyModel, Vec<ColaboratorModel>)> for Company {
    fn from(value: (CompanyModel, Vec<ColaboratorModel>)) -> Self {
        Self {
            id: Some(value.0.id),
            name: value.0.name,
            user_id: value.0.user_id,
            colaborators: value.1.into_iter().map(CompanyColaborator::from).collect(),
        }
    }
}
