use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use serde::Deserialize;

use crate::entities::location;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub fn router(state: AppState) -> Router {
    let api = Router::new()
        .route("/hello", get(hello))
        .route("/locations", get(list_locations).post(create_location))
        .route(
            "/locations/{id}",
            get(get_location).put(update_location).delete(delete_location),
        );
    Router::new().nest("/api", api).with_state(state)
}

/// Errors returned from handlers, rendered as a status code + message.
pub enum ApiError {
    NotFound,
    Db(DbErr),
}

impl From<DbErr> for ApiError {
    fn from(e: DbErr) -> Self {
        ApiError::Db(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found").into_response(),
            ApiError::Db(e) => {
                tracing::error!("database error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
            }
        }
    }
}

type ApiResult<T> = Result<T, ApiError>;

async fn hello() -> &'static str {
    "Hello from the Axum API!"
}

/// Request body for creating/updating a location.
#[derive(Deserialize)]
pub struct LocationInput {
    pub name: String,
    pub address: String,
}

async fn list_locations(State(s): State<AppState>) -> ApiResult<Json<Vec<location::Model>>> {
    Ok(Json(location::Entity::find().all(&s.db).await?))
}

async fn get_location(
    State(s): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResult<Json<location::Model>> {
    location::Entity::find_by_id(id)
        .one(&s.db)
        .await?
        .map(Json)
        .ok_or(ApiError::NotFound)
}

async fn create_location(
    State(s): State<AppState>,
    Json(input): Json<LocationInput>,
) -> ApiResult<(StatusCode, Json<location::Model>)> {
    let model = location::ActiveModel {
        name: Set(input.name),
        address: Set(input.address),
        ..Default::default()
    }
    .insert(&s.db)
    .await?;
    Ok((StatusCode::CREATED, Json(model)))
}

async fn update_location(
    State(s): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<LocationInput>,
) -> ApiResult<Json<location::Model>> {
    let existing = location::Entity::find_by_id(id)
        .one(&s.db)
        .await?
        .ok_or(ApiError::NotFound)?;
    let mut active: location::ActiveModel = existing.into();
    active.name = Set(input.name);
    active.address = Set(input.address);
    Ok(Json(active.update(&s.db).await?))
}

async fn delete_location(State(s): State<AppState>, Path(id): Path<i64>) -> ApiResult<StatusCode> {
    let res = location::Entity::delete_by_id(id).exec(&s.db).await?;
    if res.rows_affected == 0 {
        Err(ApiError::NotFound)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
