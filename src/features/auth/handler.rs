use askama::Template;
use axum::{
    Form, Router,
    response::{Html, Redirect},
    routing::{get, post},
};

use crate::{
    features::auth::request::{SignInFormRequest, SignUpFormRequest},
    shared::context::AppContext,
};

#[derive(Template)]
#[template(path = "features/auth/pages/sign-in.html.askama")]
pub struct SignInPage;

#[derive(Template)]
#[template(path = "features/auth/pages/sign-up.html.askama")]
pub struct SignUpPage;

#[axum::debug_handler]
async fn sign_in_page() -> Html<String> {
    Html(SignInPage.render().unwrap())
}

#[axum::debug_handler]
async fn sign_in_submit(Form(payload): Form<SignInFormRequest>) -> Redirect {
    println!("Sign In Payload: {:?}", payload);
    Redirect::to("/")
}

#[axum::debug_handler]
async fn sign_up_page() -> Html<String> {
    Html(SignUpPage.render().unwrap())
}

#[axum::debug_handler]
async fn sign_up_submit(Form(payload): Form<SignUpFormRequest>) -> Redirect {
    println!("Sign Up Payload: {:?}", payload);
    Redirect::to("/")
}

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/sign-in", get(sign_in_page))
        .route("/sign-in", post(sign_in_submit))
        .route("/sign-up", get(sign_up_page))
        .route("/sign-up", post(sign_up_submit))
}
