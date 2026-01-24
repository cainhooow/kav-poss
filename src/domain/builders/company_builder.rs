use crate::domain::entities::company::NewCompany;

pub struct CompanyBuilder {
    name: String,
    user_id: i32,
}

impl CompanyBuilder {
    pub fn new(name: impl Into<String>, user_id: i32) -> Self {
        Self {
            name: name.into(),
            user_id,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn user(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn build(self) -> NewCompany {
        NewCompany {
            name: self.name,
            user_id: self.user_id,
        }
    }
}
