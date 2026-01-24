use crate::{
    application::exceptions::AppResult,
    domain::{entities::company::Company, repositories::company_repository::CompanyRepository},
};

pub struct FindCompanyById<R: CompanyRepository> {
    repository: R,
}

impl<R: CompanyRepository> FindCompanyById<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, id: i32) -> AppResult<Company> {
        let company = self.repository.find_by_id(id).await?;
        Ok(company)
    }
}

pub struct FindCompanyByUserId<R: CompanyRepository> {
    repository: R,
}

impl<R: CompanyRepository> FindCompanyByUserId<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, user_id: i32) -> AppResult<Company> {
        let company = self.repository.find_by_user_id(user_id).await?;
        Ok(company)
    }
}
