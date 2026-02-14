use std::sync::Arc;

use core_server::RoleEnum;
use garde::Validate;
use salvo::{Depot, Request, Response, handler, http::StatusCode};
use uuid::Uuid;

use crate::{
    application::{
        exceptions::{AppError, AppResult},
        queries::{company_query::FindCompanyByIdQuery, user_query::FindUserByIdQuery},
        usecases::company_usecases::{
            CreateColaboratorUseCase, CreateCompanyRoleUseCase, CreateCompanyUseCase,
        },
    },
    infrastructure::{
        http::State,
        interfaces::http::resources::{
            DataResponse,
            colaborator_resource::{ColaboratorRequest, ColaboratorResource},
            company_resource::{CompanyRequest, CompanyResource},
            company_role_resource::{CompanyRoleRequest, CompanyRoleResource},
        },
        persistence::{
            sea_orm_colaborator_repository::SeaOrmColaboratorRepository,
            sea_orm_company_repository::SeaOrmCompanyRepository,
            sea_orm_company_role_repository::SeaOrmCompanyRoleRepository,
            sea_orm_user_repository::SeaOrmUserRepository,
        },
    },
};

#[handler]
pub async fn create_company_handler(
    _req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> AppResult<()> {
    let state = depot.obtain::<Arc<State>>().unwrap().to_owned();

    match _req.parse_json::<CompanyRequest>().await {
        Ok(data) => {
            let repository = SeaOrmCompanyRepository::new(state.db.clone());
            let user_repository = SeaOrmUserRepository::new(state.db.clone());

            let user_id = depot
                .get::<i32>("user_id")
                .map_err(|_| AppError::Unauthorized(String::from("Usuário não autenticado")))?;
            let user = FindUserByIdQuery::new(user_repository)
                .execute(*user_id)
                .await?;

            // if !user.roles.contains(&RoleEnum::CanCreateCompany) {
            //     res.status_code(StatusCode::FORBIDDEN);
            //     res.render(DataResponse::error(
            //         "Você não tem permissão para este recurso.",
            //     ));
            //     return Ok(());
            // }

            data.validate()
                .map_err(|err| AppError::Bad(err.to_string()))?;

            match CreateCompanyUseCase::new(repository)
                .execute(data.name, *user_id)
                .await
            {
                Ok(company) => {
                    res.status_code(StatusCode::CREATED);
                    res.render(DataResponse::success(CompanyResource::from(company)));
                }
                Err(err) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(DataResponse::error(err.to_string()));
                }
            }
        }
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(DataResponse::error(err.to_string()));
        }
    }

    Ok(())
}

#[handler]
pub async fn create_company_role_handler(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> AppResult<()> {
    let state = depot.obtain::<Arc<State>>().unwrap().to_owned();

    match req.parse_json::<CompanyRoleRequest>().await {
        Ok(data) => {
            let repository = SeaOrmCompanyRoleRepository::new(state.db.clone());
            let company_id = req
                .params()
                .get("company_id")
                .cloned()
                .unwrap()
                .parse::<i32>()?;

            data.validate()
                .map_err(|err| AppError::Bad(err.to_string()))?;

            match CreateCompanyRoleUseCase::new(repository)
                .execute(data.name, data.description, company_id)
                .await
            {
                Ok(data) => {
                    res.status_code(StatusCode::CREATED);
                    res.render(DataResponse::success(CompanyRoleResource::from(data)));
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

#[handler]
pub async fn get_company_roles_handler(req: &mut Request, res: &mut Response, depot: &mut Depot) {}

#[handler]
pub async fn create_company_colaborator_handler(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> AppResult<()> {
    let state = depot.obtain::<Arc<State>>().unwrap().to_owned();

    match req.parse_json::<ColaboratorRequest>().await {
        Ok(data) => {
            let repository = SeaOrmColaboratorRepository::new(state.db.clone());
            let company_repository = SeaOrmCompanyRepository::new(state.db.clone());
            let user_respository = SeaOrmUserRepository::new(state.db.clone());

            let company_id = req
                .params()
                .get("company_id")
                .cloned()
                .unwrap()
                .parse::<i32>()?;

            FindCompanyByIdQuery::new(company_repository)
                .execute(company_id)
                .await
                .map_err(|_| {
                    AppError::Bad(String::from(
                        "Compania invalida ou não encontrada na nossa base de dados",
                    ))
                })?;

            data.validate()
                .map_err(|err| AppError::Bad(err.to_string()))?;

            FindUserByIdQuery::new(user_respository)
                .execute(data.user_id.unwrap())
                .await
                .map_err(|_| {
                    AppError::Bad(String::from(
                        "Usuário inválido ou não encontrado na nossa base de dados",
                    ))
                })?;

            match CreateColaboratorUseCase::new(repository)
                .execute(
                    data.document,
                    Uuid::new_v4().to_string(),
                    company_id,
                    data.user_id.unwrap(),
                )
                .await
            {
                Ok(data) => {
                    res.status_code(StatusCode::CREATED);
                    res.render(DataResponse::success(ColaboratorResource::from(data)));
                }
                Err(err) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(DataResponse::error(err.to_string()));
                }
            }
        }
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(DataResponse::error(err.to_string()));
        }
    }

    Ok(())
}

#[handler]
pub async fn get_company_colaborators_handler(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) {
}
