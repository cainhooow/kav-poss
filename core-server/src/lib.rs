use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::str::FromStr;
#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, PartialEq, EnumString, EnumIter, Display, Serialize, Deserialize)]
pub enum RoleEnum {
    /** USER PERMISSION */
    CanAuthenticate,
    CanCreateSalePoint,
    CanUpdateSalePoint,
    CanModifySalePoint,
    CanCreateCompany,
    CanUpdateCompany,
    CanModifyCompany,
    CanManageUsersSalePoints,
    CanCreateProduct,
    CanUpdateProduct,
    CanDeleteProduct,
    /** SYS PERMISSION */
    CanCreateUser,
    CanUpdateUser,
    CanDeleteUser,
    CanAssignUserRole,
    CanCreateApiPlan,
    CanModifyApiPlan,
    CanAssignApiPlan,
}
