use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::domain::entities::company::Company;

#[derive(Serialize, Deserialize, Validate)]
pub struct CompanyRequest {
    #[garde(ascii, length(min = 5))]
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompanyResource {
    pub name: String,
}

impl From<&Company> for CompanyResource {
    fn from(value: &Company) -> Self {
        Self {
            name: value.name.clone(),
        }
    }
}

impl From<Company> for CompanyResource {
    fn from(value: Company) -> Self {
        Self { name: value.name }
    }
}

impl CompanyResource {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
