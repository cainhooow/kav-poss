use crate::{
    application::exceptions::AppResult,
    domain::{
        builders::company_builder::CompanyBuilder,
        entities::{
            company::Company,
            company_colaborator::{ColaboratorBuilder, CompanyColaborator},
            company_role::{CompanyRole, CompanyRoleBuilder},
        },
        repositories::{
            colaborator_repository_interface::ColaboratorRepository,
            company_repository_interface::CompanyRepository,
            company_role_repository_interface::CompanyRoleRepository,
        },
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

pub struct CreateColaboratorUseCase<R: ColaboratorRepository> {
    repository: R,
}

impl<R: ColaboratorRepository> CreateColaboratorUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(
        &self,
        document: String,
        badge: String,
        company_id: i32,
        user_id: i32,
    ) -> AppResult<CompanyColaborator> {
        let colaborator = ColaboratorBuilder::new(document, badge, user_id, company_id).build();

        Ok(self.repository.save(&colaborator).await?)
    }
}

pub struct CreateCompanyRoleUseCase<R: CompanyRoleRepository> {
    repository: R,
}

impl<R: CompanyRoleRepository> CreateCompanyRoleUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(
        &self,
        name: String,
        description: Option<String>,
        company_id: i32,
    ) -> AppResult<CompanyRole> {
        let role = CompanyRoleBuilder::new(name, company_id)
            .description(description.unwrap_or(String::from("")))
            .build();

        Ok(self.repository.save(&role).await?)
    }
}
