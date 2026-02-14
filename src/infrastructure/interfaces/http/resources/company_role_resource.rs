use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::domain::entities::company_role::CompanyRole;

#[derive(Serialize, Deserialize, Validate)]
pub struct CompanyRoleRequest {
    #[garde(ascii)]
    pub name: String,
    #[garde(required)]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CompanyRoleResource {
    pub name: String,
    pub description: Option<String>,
}

impl From<&CompanyRole> for CompanyRoleResource {
    fn from(value: &CompanyRole) -> Self {
        Self {
            name: value.name.clone(),
            description: value.description.clone(),
        }
    }
}

impl From<CompanyRole> for CompanyRoleResource {
    fn from(value: CompanyRole) -> Self {
        Self {
            name: value.name,
            description: value.description,
        }
    }
}
