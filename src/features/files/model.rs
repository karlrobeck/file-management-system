use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Folder {
    id: i64,
    user_id: i64,
    name: String,
    parent_folder_id: Option<i64>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, FromRow)]
pub struct File {
    pub id: i64,
    pub user_id: i64,
    pub folder_id: Option<i64>,
    pub name: String,
    pub storage_path: String,
    pub size_bytes: i64,
    pub mime_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
