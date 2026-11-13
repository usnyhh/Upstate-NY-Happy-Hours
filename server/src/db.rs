//! Database connection, schema setup, and sample data.

use sea_orm::{
    ActiveModelTrait, ConnectionTrait, Database, DatabaseConnection, DbErr, EntityTrait,
    PaginatorTrait, Schema, Set,
};

use crate::entities::location;

/// `(name, address)` rows inserted into an empty `locations` table.
const SEED_LOCATIONS: &[(&str, &str)] = &[
    ("Example Tavern", "123 Main St, Albany, NY"),
    ("Sample Warehouse", "456 State St, Troy, NY"),
];

/// Connect to the database, create missing tables, and seed sample data.
pub async fn connect(url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(url).await?;
    create_tables(&db).await?;
    seed(&db).await?;
    Ok(db)
}

/// Create each entity's table if it doesn't exist yet. Good enough while the
/// schema is small; switch to `sea-orm-migration` once it starts changing.
async fn create_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let schema = Schema::new(db.get_database_backend());
    let mut stmt = schema.create_table_from_entity(location::Entity);
    stmt.if_not_exists();
    db.execute(&stmt).await?;
    Ok(())
}

/// Insert `SEED_LOCATIONS`, but only into an empty table.
async fn seed(db: &DatabaseConnection) -> Result<(), DbErr> {
    if location::Entity::find().count(db).await? > 0 {
        return Ok(());
    }
    for &(name, address) in SEED_LOCATIONS {
        location::ActiveModel {
            name: Set(name.into()),
            address: Set(address.into()),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }
    Ok(())
}
