use super::{connection, question};
use sea_orm::ConnectionTrait;
use sea_orm::Schema;

//TODO add other tables
pub async fn create_tables() {
    let conn = connection::connect().await.unwrap();

    let backend = conn.get_database_backend();

    let schema = Schema::new(backend);

    let mut stmt = schema.create_table_from_entity(question::Entity);

    stmt.if_not_exists();

    match conn.execute(backend.build(&stmt)).await {
        Ok(_) => {}
        Err(error) => {
            panic!("{error}");
        }
    }
}
