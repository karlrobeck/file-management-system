use askama::Template;
use axum::{Router, middleware, response::Html, routing::get};

use crate::{features::auth::extractor::Authenticated, shared::context::AppContext};

#[derive(Template)]
#[template(path = "features/files/pages/index.html.askama")]
pub struct FilePage;

pub async fn file_page() -> Html<String> {
    let page = FilePage;
    Html(page.render().unwrap())
}

pub fn router(state: &AppContext) -> Router<AppContext> {
    Router::new()
        .route("/", get(file_page))
        .layer(middleware::from_extractor_with_state::<Authenticated, _>(
            state.clone(),
        ))
}
