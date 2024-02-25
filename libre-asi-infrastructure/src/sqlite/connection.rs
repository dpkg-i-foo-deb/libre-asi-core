use super::SqliteError;
use sea_orm::{DatabaseConnection, SqlxSqliteConnector};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const DB_FILE: &str = "database.db";

pub async fn init_db() -> Result<(), SqliteError> {
    match Sqlite::database_exists(DB_FILE).await? {
        true => {
            println!("SQLite database already exists")
        }
        false => {
            println!("Creating database");
            Sqlite::create_database(DB_FILE).await?
        }
    }

    Ok(())
}

pub async fn connect() -> Result<DatabaseConnection, SqliteError> {
    let pool = SqlitePool::connect(DB_FILE).await?;

    let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);

    Ok(conn)
}
