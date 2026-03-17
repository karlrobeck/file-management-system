use askama::Template;
use axum::{Router, extract::State, middleware, response::Html, routing::get};

use crate::{
    features::{
        auth::{extractor::Authenticated, model::Session},
        files::model::File,
    },
    shared::context::AppContext,
};

#[derive(Template)]
#[template(path = "features/files/pages/index.html.askama")]
pub struct FilePage {
    files: Vec<File>,
}

pub async fn file_page(State(state): State<AppContext>, session: Session) -> Html<String> {
    let files = sqlx::query_as::<_, File>(
        r#"
        select * from files where user_id = ?
    "#,
    )
    .bind(session.user_id)
    .fetch_all(&state.db_pool)
    .await
    .unwrap();

    let page = FilePage { files };

    Html(page.render().unwrap())
}

pub fn router(state: &AppContext) -> Router<AppContext> {
    Router::new()
        .route("/", get(file_page))
        .layer(middleware::from_extractor_with_state::<Authenticated, _>(
            state.clone(),
        ))
}
