use axum::{
    Router,
    extract::{Multipart, State},
    response::Redirect,
    routing::post,
};

use crate::{
    features::{auth::model::Session, files::model::File},
    shared::context::AppContext,
};

#[axum::debug_handler]
async fn upload_file(
    State(state): State<AppContext>,
    session: Session,
    mut multipart: Multipart,
) -> Redirect {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let mime_type = field.content_type().unwrap().to_string();
        let bytes = field.bytes().await.unwrap();

        let mut trx = state.db_pool.begin().await.unwrap();

        let file_path = format!("./uploads/{}", file_name);

        let file = sqlx::query_as::<_,File>(r#"
            insert into files (user_id, name, storage_path, size_bytes, mime_type) values (?, ?, ?, ?, ?) returning *
        "#)
        .bind(session.user_id)
        .bind(file_name)
        .bind(file_path)
        .bind(bytes.len() as i64)
        .bind(mime_type)
        .fetch_one(&mut *trx)
        .await
        .unwrap();

        if let Err(e) = tokio::fs::write(&file.storage_path, bytes).await {
            trx.rollback().await.unwrap();
            eprintln!("Failed to save file: {}", e);
        } else {
            trx.commit().await.unwrap();
        }

        println!("Uploaded File: {:?}", file);
    }
    Redirect::to("/")
}

pub fn router() -> Router<AppContext> {
    Router::new().route("/upload", post(upload_file))
}
