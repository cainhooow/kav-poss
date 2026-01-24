use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use core_server::RoleEnum;
use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::{
        exceptions::AppResult,
        queries::user_query::{FindUserByEmailQuery, FindUserByIdQuery},
        usecases::user_usecases::CreateUserWithRolesUseCase,
    },
    infrastructure::{
        http::State,
        interfaces::http::resources::{
            DataResponse,
            auth_resource::{AuthRefreshResource, AuthRequest, AuthResource, RefreshTokenRequest},
            user_resource::{UserRequest, UserResource},
        },
        persistence::{
            sea_orm_role_repository::SeaOrmRoleRepository,
            sea_orm_user_repository::SeaOrmUserRepository,
        },
    },
};

fn refresh_token(state: Arc<State>, refresh_token: &str, res: &mut Response) -> AppResult<()> {
    let new_token = state.auth_service.refresh_access_token(refresh_token)?;
    state
        .cookie_service
        .generate_sessions(&new_token, refresh_token, res);

    res.render(DataResponse::success(AuthRefreshResource {
        refresh_token: refresh_token.to_string(),
        access_token: new_token,
    }));
    res.status_code(StatusCode::OK);
    Ok(())
}

#[handler]
pub async fn auth_user(_req: &mut Request, res: &mut Response, depot: &mut Depot) -> AppResult<()> {
    let state = depot.obtain::<Arc<State>>().unwrap().to_owned();
    let user_repository = SeaOrmUserRepository::new(state.db.clone());

    match depot.get::<i32>("user_id") {
        Ok(user_id) => {
            match FindUserByIdQuery::new(user_repository)
                .execute(*user_id)
                .await
            {
                Ok(user) => {
                    res.render(DataResponse::success(UserResource::from(&user)));
                    res.status_code(StatusCode::OK);
                }
                Err(err) => {
                    res.render(DataResponse::error(err.to_string()));
                    res.status_code(StatusCode::UNAUTHORIZED);
                }
            }
        }
        Err(_) => {
            res.render(DataResponse::error("No Auth user"));
            res.status_code(StatusCode::UNAUTHORIZED);
        }
    }

    Ok(())
}

#[handler]
pub async fn auth_local_refresh(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> AppResult<()> {
    let state = depot.obtain::<Arc<State>>().unwrap().to_owned();

    if let Some(cookie) = req.cookie("kvrefresh") {
        refresh_token(state, cookie.value(), res)?;
    } else {
        match req.parse_json::<RefreshTokenRequest>().await {
            Ok(auth_body) => {
                refresh_token(state, &auth_body.refresh_token, res)?;
            }
            Err(err) => {
                res.render(DataResponse::error(err.to_string()));
                res.status_code(StatusCode::BAD_REQUEST);
            }
        }
    }

    Ok(())
}

#[handler]
pub async fn auth_local_login(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> AppResult<()> {
    let state = depot.obtain::<Arc<State>>().unwrap().to_owned();

    match req.parse_json::<AuthRequest>().await {
        Ok(req) => {
            let user_respository = SeaOrmUserRepository::new(state.db.clone());

            match FindUserByEmailQuery::new(user_respository)
                .execute(req.email)
                .await
            {
                Ok(user) => {
                    let argon2 = Argon2::default();
                    let password_hash = PasswordHash::new(&user.password)?;
                    argon2.verify_password(req.password.as_bytes(), &password_hash)?;
                    let tokens = state.auth_service.generate(user.id.unwrap())?;

                    let _ = state.cookie_service.generate_sessions(
                        &tokens.access_token,
                        &tokens.refresh_token,
                        res,
                    );

                    res.render(DataResponse::success(AuthResource::new(
                        user,
                        tokens.access_token,
                        tokens.refresh_token,
                    )))
                }
                Err(_) => {
                    res.status_code(StatusCode::UNAUTHORIZED);
                    res.render(DataResponse::error(String::from(
                        "Invalid user email/username or password",
                    )))
                }
            }
        }
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(DataResponse::error(err.to_string()))
        }
    }

    Ok(())
}

#[handler]
pub async fn auth_local_register(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> AppResult<()> {
    let state = depot.obtain::<Arc<State>>().unwrap();

    match req.parse_json::<UserRequest>().await {
        Ok(req) => {
            req.validate()?;
            let user_repository = SeaOrmUserRepository::new(state.db.clone());
            let role_repository = SeaOrmRoleRepository::new(state.db.clone());

            match CreateUserWithRolesUseCase::new(user_repository, role_repository)
                .execute(
                    req.name,
                    req.email,
                    &req.password,
                    vec![RoleEnum::CanAuthenticate],
                )
                .await
            {
                Ok(data) => {
                    res.status_code(StatusCode::CREATED);
                    res.render(DataResponse::success(UserResource::from(&data)));
                }
                Err(err) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(DataResponse::error(err.to_string()));
                }
            }
        }
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(DataResponse::error(err.to_string()))
        }
    }

    Ok(())
}
