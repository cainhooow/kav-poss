use std::sync::Arc;

use core_server::RoleEnum;
use garde::Validate;
use salvo::{Depot, Request, Response, handler, http::StatusCode};

use crate::{
    application::{
        exceptions::{AppError, AppResult},
        queries::user_query::FindUserByIdQuery,
        usecases::company_usecases::CreateCompanyUseCase,
    },
    infrastructure::{
        http::State,
        interfaces::http::resources::{
            DataResponse,
            company_resource::{CompanyRequest, CompanyResource},
        },
        persistence::{
            sea_orm_company_repository::SeaOrmCompanyRepository,
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

            if !user.roles.contains(&RoleEnum::CanCreateCompany) {
                res.status_code(StatusCode::FORBIDDEN);
                res.render(DataResponse::error(
                    "Você não tem permissão para este recurso.",
                ));
                return Ok(());
            }

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
