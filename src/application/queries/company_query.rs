use crate::{
    application::exceptions::AppResult,
    domain::{
        entities::{
            company::Company, company_colaborator::CompanyColaborator, company_role::CompanyRole,
        },
        repositories::{
            colaborator_repository_interface::ColaboratorRepository,
            company_repository_interface::CompanyRepository,
            company_role_repository_interface::CompanyRoleRepository,
        },
    },
};

pub struct FindCompanyByIdQuery<R: CompanyRepository> {
    repository: R,
}

impl<R: CompanyRepository> FindCompanyByIdQuery<R> {
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

pub struct GetCompanyColaboratorsQuery<R: ColaboratorRepository> {
    repository: R,
}

impl<R: ColaboratorRepository> GetCompanyColaboratorsQuery<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, company_id: i32) -> AppResult<Vec<CompanyColaborator>> {
        let colaborators = self.repository.all(company_id).await?;
        Ok(colaborators)
    }
}

pub struct FindCompanyColaboratorByUserId<R: ColaboratorRepository> {
    repository: R,
}

impl<R: ColaboratorRepository> FindCompanyColaboratorByUserId<R> {
    pub fn new(repo: R) -> Self {
        Self {
            repository: repo
        }
    }

    pub async fn execute(&self, company_id: i32, user_id: i32) -> AppResult<CompanyColaborator> {
        let colaborator = self.repository.find_by_user_id(company_id, user_id).await?;
        Ok(colaborator)
    }
}

pub struct GetCompanyRolesQuery<R: CompanyRoleRepository> {
    repository: R,
}

impl<R: CompanyRoleRepository> GetCompanyRolesQuery<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, company_id: i32) -> AppResult<Vec<CompanyRole>> {
        let roles = self.repository.all(company_id).await?;
        Ok(roles)
    }
}
