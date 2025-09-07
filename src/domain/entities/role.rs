#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/** examples roles */
pub enum Roles {
    CanAddBussines,
    CanUpdateBussines,
    CanModifyBussines,
}

#[derive(Debug, Clone)]
pub struct Role {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct NewRole {
    pub name: String,
    pub description: Option<String>,
}

impl Roles {
    pub fn as_str(&self) -> &'static str {
        match self {
            Roles::CanAddBussines => "CAN_ADD_BUSSINESS",
            Roles::CanUpdateBussines => "CAN_UPDATE_BUSSINES",
            Roles::CanModifyBussines => "CAN_MODIFY_BUSSINES",
        }
    }

    pub fn from_str(readable_role: &str) -> Option<Self> {
        match readable_role {
            "CAN_ADD_BUSSINES" => Some(Roles::CanAddBussines),
            "CAN_UPDATE_BUSSINES" => Some(Roles::CanUpdateBussines),
            "CAN_MODIFY_BUSSINES" => Some(Roles::CanModifyBussines),
            _ => None,
        }
    }
}
