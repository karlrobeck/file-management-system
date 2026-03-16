use askama::Template;
use axum::{
    Form, Router,
    extract::State,
    response::{Html, Redirect},
    routing::{get, post},
};

use crate::{
    features::auth::{
        model::User,
        request::{SignInFormRequest, SignUpFormRequest},
    },
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
async fn sign_in_submit(
    State(state): State<AppContext>,
    Form(payload): Form<SignInFormRequest>,
) -> Redirect {
    let current_user = sqlx::query_as::<_, User>(
        r#"
        select * from users where username = ? and password_hash = ?
    "#,
    )
    .bind(&payload.username)
    .bind(&payload.password)
    .fetch_optional(&state.db_pool)
    .await
    .unwrap();

    if let Some(user) = current_user {
        return Redirect::to("/");
    }

    Redirect::to("/auth/sign-in")
}

#[axum::debug_handler]
async fn sign_up_page() -> Html<String> {
    Html(SignUpPage.render().unwrap())
}

#[axum::debug_handler]
async fn sign_up_submit(
    State(state): State<AppContext>,
    Form(payload): Form<SignUpFormRequest>,
) -> Redirect {
    let trx = state.db_pool.begin().await.unwrap();

    let new_user = sqlx::query_as::<_, User>(
        r#"
        insert into users (username,password_hash) values (?,?) returning *
    "#,
    )
    .bind(&payload.username)
    .bind(&payload.password)
    .fetch_one(&state.db_pool)
    .await
    .unwrap();

    println!("New User: {:?}", new_user);

    trx.commit().await.unwrap();

    Redirect::to("/")
}

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/sign-in", get(sign_in_page))
        .route("/sign-in", post(sign_in_submit))
        .route("/sign-up", get(sign_up_page))
        .route("/sign-up", post(sign_up_submit))
}
