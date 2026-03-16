use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "features/files/pages/index.html.askama")]
pub struct FilePage;

pub async fn file_page() -> Html<String> {
    let page = FilePage;
    Html(page.render().unwrap())
}
