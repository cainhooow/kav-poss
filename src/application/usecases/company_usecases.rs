use crate::{
    application::exceptions::AppResult,
    domain::{
        builders::company_builder::CompanyBuilder, entities::company::Company,
        repositories::company_repository::CompanyRepository,
    },
};

pub struct CreateCompanyUseCase<R: CompanyRepository> {
    repository: R,
}

impl<R: CompanyRepository> CreateCompanyUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, name: String, user_id: i32) -> AppResult<Company> {
        let company = CompanyBuilder::new(name, user_id).build();

        Ok(self.repository.save(&company).await?)
    }
}
