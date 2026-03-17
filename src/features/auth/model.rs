use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    #[sqlx(rename = "storage_quota_bytes")]
    pub storage_quota: i64,
    #[sqlx(rename = "storage_used_bytes")]
    pub storage_used: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
#[derive(Debug, Clone, FromRow)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
