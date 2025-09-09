use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, EnumString, EnumIter, Display, Serialize, Deserialize)]
pub enum RoleEnum {
    CanCreateBussines,
    CanUpdateBussines,
    CanModifyBussines,
    CanCreateSalePoint,
    CanUpdateSalePoint,
    CanModifySalePoint,
    CanManageUsersSalePoints,
}
