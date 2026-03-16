#[derive(Clone)]
pub struct AppContext {
    pub db_pool: sqlx::SqlitePool,
}
