#[derive(Debug, Clone)]
pub struct Role {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
}

pub struct RoleBuilder {
    name: String,
    description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct NewRole {
    pub name: String,
    pub description: Option<String>,
}

impl RoleBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn build(self) -> NewRole {
        NewRole {
            name: self.name,
            description: self.description,
        }
    }
}
