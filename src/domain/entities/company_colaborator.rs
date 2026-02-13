#[derive(Debug, Clone)]
pub struct CompanyColaborator {
    pub id: Option<i32>,
    pub company_id: i32,
    pub user_id: i32,
    pub document: String,
    pub badge: String,
}

pub struct ColaboratorBuilder {
    company_id: i32,
    user_id: i32,
    document: String,
    badge: String,
}

#[derive(Debug, Clone)]
pub struct NewColaborator {
    pub company_id: i32,
    pub user_id: i32,
    pub document: String,
    pub badge: String,
}

impl ColaboratorBuilder {
    pub fn new(
        document: impl Into<String>,
        badge: impl Into<String>,
        user_id: i32,
        company_id: i32,
    ) -> Self {
        Self {
            company_id,
            user_id,
            document: document.into(),
            badge: badge.into(),
        }
    }

    pub fn document(mut self, document: impl Into<String>) -> Self {
        self.document = document.into();
        self
    }

    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = badge.into();
        self
    }

    pub fn user(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn company(mut self, company_id: i32) -> Self {
        self.company_id = company_id;
        self
    }

    pub fn build(self) -> NewColaborator {
        NewColaborator {
            company_id: self.company_id,
            user_id: self.user_id,
            document: self.document,
            badge: self.badge,
        }
    }
}
