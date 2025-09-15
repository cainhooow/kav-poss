use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::str::FromStr;
#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, EnumString, EnumIter, Display, Serialize, Deserialize)]
pub enum RoleEnum {
    CanAuthenticate,
    CanCreateUser,
    CanUpdateUser,
    CanDeleteUser,
    CanAssignUserRole,
    CanCreateSalePoint,
    CanUpdateSalePoint,
    CanModifySalePoint,
    CanCreateBussines,
    CanUpdateBussines,
    CanModifyBussines,
    CanManageUsersSalePoints,
    CanCreateProduct,
    CanUpdateProduct,
    CanDeleteProduct,
}
