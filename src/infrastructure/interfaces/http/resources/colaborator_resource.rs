use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::domain::entities::company_colaborator::CompanyColaborator;

#[derive(Serialize, Deserialize, Validate)]
pub struct ColaboratorRequest {
    #[garde(ascii)]
    pub document: String,
    #[garde(required)]
    pub user_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ColaboratorResource {
    pub id: Option<i32>,
    pub badge: String,
}

impl From<&CompanyColaborator> for ColaboratorResource {
    fn from(value: &CompanyColaborator) -> Self {
        Self {
            id: value.id.clone(),
            badge: value.badge.clone(),
        }
    }
}

impl From<CompanyColaborator> for ColaboratorResource {
    fn from(value: CompanyColaborator) -> Self {
        Self {
            id: value.id,
            badge: value.badge,
        }
    }
}

impl ColaboratorResource {
    pub fn collection(items: Vec<CompanyColaborator>) -> Vec<Self> {
        items.into_iter().map(ColaboratorResource::from).collect()
    }
}