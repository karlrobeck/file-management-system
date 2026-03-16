use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    id: i32,
    username: String,
    password_hash: String,
    #[sqlx(rename = "storage_quota_bytes")]
    storage_quota: i64,
    #[sqlx(rename = "storage_used_bytes")]
    storage_used: i64,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}
