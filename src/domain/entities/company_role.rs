use core_server::RoleEnum;

use crate::domain::entities::role::Role;

#[derive(Debug, Clone)]
pub struct CompanyRole {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub company_id: i32,
    pub flags: Vec<Role>,
}

pub struct CompanyRoleBuilder {
    name: String,
    description: Option<String>,
    company_id: i32,
}

pub struct NewCompanyRole {
    pub name: String,
    pub description: Option<String>,
    pub company_id: i32,
}

impl CompanyRoleBuilder {
    pub fn new(name: impl Into<String>, company_id: i32) -> Self {
        Self {
            name: name.into(),
            description: None,
            company_id: company_id,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = self.description;
        self
    }

    pub fn company(mut self, company_id: i32) -> Self {
        self.company_id = company_id;
        self
    }

    pub fn build(self) -> NewCompanyRole {
        NewCompanyRole {
            name: self.name,
            description: self.description,
            company_id: self.company_id,
        }
    }
}
