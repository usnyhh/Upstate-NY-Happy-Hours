use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// A bar/restaurant. Serialized in camelCase to match the client's
/// `Location` struct (`client/src/app.rs`): `locationId`, `name`, `address`.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "locations")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(rename = "locationId")]
    pub id: i64,
    pub name: String,
    pub address: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
