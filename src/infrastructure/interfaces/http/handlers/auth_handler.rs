use std::sync::Arc;

use core_server::RoleEnum;
use salvo::prelude::*;

use crate::{
    application::{exceptions::AppResult, usecases::user_usecases::CreateUserWithRolesUseCase},
    infrastructure::{
        http::State,
        interfaces::http::resources::{
            DataResponse,
            user_resource::{UserRequest, UserResource},
        },
        persistence::{
            sea_orm_role_repository::SeaOrmRoleRepository,
            sea_orm_user_repository::SeaOrmUserRepository,
        },
    },
};

#[handler]
pub async fn auth_local(req: &mut Request, res: &mut Response, depot: &mut Depot) {}

#[handler]
pub async fn auth_local_register(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> AppResult<()> {
    let state = depot.obtain::<Arc<State>>().unwrap();

    match req.parse_json::<UserRequest>().await {
        Ok(req) => {
            let user_repository = SeaOrmUserRepository::new(state.db.clone());
            let role_repository = SeaOrmRoleRepository::new(state.db.clone());

            match CreateUserWithRolesUseCase::new(user_repository, role_repository)
                .execute(
                    req.name,
                    req.email,
                    &req.password,
                    vec![RoleEnum::CanCreateBussines],
                )
                .await
            {
                Ok(data) => {
                    res.status_code(StatusCode::CREATED);
                    res.render(Json(DataResponse::success(UserResource::from(&data))));
                }
                Err(err) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(DataResponse::error(err.to_string())));
                }
            }
        }
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(DataResponse::error(err.to_string())))
        }
    }

    Ok(())
}
