use sea_orm::{
    ActiveModelTrait, ConnectionTrait, Database, DatabaseConnection, DbErr, EntityTrait,
    PaginatorTrait, Schema, Set,
};

use crate::entities::location;

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
    let backend = db.get_database_backend();
    let schema = Schema::new(backend);
    let mut stmt = schema.create_table_from_entity(location::Entity);
    stmt.if_not_exists();
    db.execute(&stmt).await?;
    Ok(())
}

async fn seed(db: &DatabaseConnection) -> Result<(), DbErr> {
    if location::Entity::find().count(db).await? > 0 {
        return Ok(());
    }
    for (name, address) in [
        ("Example Tavern", "123 Main St, Albany, NY"),
        ("Sample Warehouse", "456 State St, Troy, NY"),
    ] {
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
