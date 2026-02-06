use std::sync::Arc;

use garde::Validate;
use salvo::{Depot, Request, Response, handler, http::StatusCode};

use crate::{
    application::{
        exceptions::{AppError, AppResult},
        queries::plan_query::GetPlanFeaturesQuery,
        usecases::plan_usecases::CreatePlanUseCase,
    },
    infrastructure::{
        http::State,
        interfaces::http::resources::{
            DataResponse,
            plan_resource::{PlanRequest, PlanResource},
        },
        persistence::sea_orm_plan_repository::SeaOrmPlanRepository,
    },
};

#[handler]
pub async fn create_plan_handler(
    depot: &mut Depot,
    req: &mut Request,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<Arc<State>>().unwrap().to_owned();

    match req.parse_json::<PlanRequest>().await {
        Ok(data) => {
            let repository = SeaOrmPlanRepository::new(state.db.clone());

            data.validate()
                .map_err(|err| AppError::Bad(err.to_string()))?;

            match CreatePlanUseCase::new(repository)
                .execute(data.name, data.price, data.description)
                .await
            {
                Ok(data) => {
                    res.status_code(StatusCode::CREATED);
                    res.render(DataResponse::success(PlanResource::from(data)));
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
pub async fn update_plan_handler() {}

#[handler]
pub async fn delete_plan_handler() {}

#[handler]
pub async fn get_plan_features_handler(
    depot: &mut Depot,
    req: &mut Request,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<Arc<State>>().unwrap();
    let repository = SeaOrmPlanRepository::new(state.db.clone());

    let plan_id = req
        .params()
        .get("plan_id")
        .cloned()
        .unwrap()
        .parse::<i32>()?;

    match GetPlanFeaturesQuery::new(repository).execute(plan_id).await {
        Ok(data) => {
            res.status_code(StatusCode::OK);
            res.render(DataResponse::success(data));
        }
        Err(err) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(DataResponse::error(err.to_string()));
        }
    }

    Ok(())
}

#[handler]
pub async fn create_plan_feature_handler() {}
