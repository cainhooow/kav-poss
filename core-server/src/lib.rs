use strum_macros::{Display, EnumIter, EnumString};
use strum::IntoEnumIterator;
use std::str::FromStr;

#[derive(Debug, Clone, EnumString, EnumIter, Display)]
pub enum RoleEnum {
    CanCreateBussines,
    CanUpdateBussines,
    CanModifyBussines,
    CanCreateSalePoint,
    CanUpdateSalePoint,
    CanModifySalePoint,
    CanManageUsersSalePoints,
}