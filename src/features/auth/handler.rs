use askama::Template;
use axum::response::{Html, Redirect};

#[derive(Template)]
#[template(path = "features/auth/pages/sign-in.html.askama")]
pub struct SignInPage;

#[derive(Template)]
#[template(path = "features/auth/pages/sign-up.html.askama")]
pub struct SignUpPage;

#[axum::debug_handler]
pub async fn sign_in_page() -> Html<String> {
    Html(SignInPage.render().unwrap())
}

#[axum::debug_handler]
pub async fn sign_in_submit() -> Redirect {
    Redirect::to("/")
}

#[axum::debug_handler]
pub async fn sign_up_page() -> Html<String> {
    Html(SignUpPage.render().unwrap())
}

#[axum::debug_handler]
pub async fn sign_up_submit() -> Redirect {
    Redirect::to("/")
}
