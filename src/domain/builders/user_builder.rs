use crate::domain::entities:: user::NewUser;

pub struct RoleBuilder {
    name: String,
    description: Option<String>,
}

pub struct UserBuilder {
    name: String,
    email: String,
    password: String,
}

impl UserBuilder {
    pub fn new(
        name: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            password: password.into(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }

    pub fn build(self) -> NewUser {
        NewUser {
            name: self.name,
            email: self.email,
            password: self.password,
        }
    }
}
