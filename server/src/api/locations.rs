//! CRUD endpoints for `/api/locations`.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::Deserialize;

use super::AppState;
use super::error::{ApiError, ApiResult};
use crate::entities::location;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/locations", get(list).post(create))
        .route("/locations/{id}", get(show).put(update).delete(remove))
}

/// Request body for creating or updating a location.
#[derive(Deserialize)]
pub struct LocationInput {
    pub name: String,
    pub address: String,
}

/// `GET /locations`
async fn list(State(s): State<AppState>) -> ApiResult<Json<Vec<location::Model>>> {
    Ok(Json(location::Entity::find().all(&s.db).await?))
}

/// `GET /locations/{id}`
async fn show(State(s): State<AppState>, Path(id): Path<i64>) -> ApiResult<Json<location::Model>> {
    Ok(Json(find(&s.db, id).await?))
}

/// `POST /locations`
async fn create(
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

/// `PUT /locations/{id}`
async fn update(
    State(s): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<LocationInput>,
) -> ApiResult<Json<location::Model>> {
    let mut active: location::ActiveModel = find(&s.db, id).await?.into();
    active.name = Set(input.name);
    active.address = Set(input.address);
    Ok(Json(active.update(&s.db).await?))
}

/// `DELETE /locations/{id}`
async fn remove(State(s): State<AppState>, Path(id): Path<i64>) -> ApiResult<StatusCode> {
    let result = location::Entity::delete_by_id(id).exec(&s.db).await?;
    if result.rows_affected == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

/// Fetch one location, or `NotFound` if there's no row with that id.
async fn find(db: &DatabaseConnection, id: i64) -> ApiResult<location::Model> {
    location::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(ApiError::NotFound)
}
