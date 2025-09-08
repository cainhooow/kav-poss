use sea_orm::EnumIter;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString, EnumIter, Clone)]

pub enum RolesEnum {
    CanCreateBussines,
    CanUpdateBussines,
    CanModifyBussines,
    CanCreateSalePoint,
    CanUpdateSalePoint,
    CanModifySalePoint,
    CanManageUsersSalePoints,
}

#[derive(Debug, Clone)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct NewRole {
    pub name: String,
    pub description: Option<String>,
}

// impl RolesEnum {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             RolesEnum::CanCreateBussines => "CAN_CREATE_BUSSINESS",
//             RolesEnum::CanUpdateBussines => "CAN_UPDATE_BUSSINES",
//             RolesEnum::CanModifyBussines => "CAN_MODIFY_BUSSINES",
//         }
//     }

//     pub fn from_str(readable_role: &str) -> Option<Self> {
//         match readable_role {
//             "CAN_CREATE_BUSSINES" => Some(RolesEnum::CanCreateBussines),
//             "CAN_UPDATE_BUSSINES" => Some(RolesEnum::CanUpdateBussines),
//             "CAN_MODIFY_BUSSINES" => Some(RolesEnum::CanModifyBussines),
//             _ => None,
//         }
//     }
// }
