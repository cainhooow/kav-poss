use std::sync::Arc;

use salvo::{Depot, Request, Response, handler, http::StatusCode};

use crate::{
    application::{exceptions::AppResult, usecases::company_usecases::CreateCompanyUseCase},
    infrastructure::{
        http::State,
        interfaces::http::resources::{DataResponse, company_resource::CompanyResource},
        persistence::sea_orm_company_repository::SeaOrmCompanyRepository,
    },
};

#[handler]
pub async fn create_company_handler(_req: &mut Request, res: &mut Response, depot: &mut Depot) {
    let state = depot.obtain::<Arc<State>>().unwrap().to_owned();

    match _req.parse_json::<CompanyResource>().await {
        Ok(data) => {
            let repository = SeaOrmCompanyRepository::new(state.db.clone());
            let auth_user = depot.get::<i32>("user_id");

            if let Ok(user_id) = auth_user {
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
        }
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(DataResponse::error(err.to_string()));
        }
    }
}
