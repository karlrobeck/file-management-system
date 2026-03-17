use askama::Template;
use axum::{
    Form, Router, extract::State, middleware, response::{Html, Redirect}, routing::{get, post}
};
use axum_extra::extract::{CookieJar, PrivateCookieJar, cookie::Cookie};

use crate::{
    features::auth::{
         extractor::Authenticated, model::{Session, User}, request::{SignInFormRequest, SignUpFormRequest}
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
    jar: PrivateCookieJar,
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
        let new_session = sqlx::query_as::<_, Session>(
            r#"
            insert into sessions (user_id, token, expires_at) values (?, ?, datetime('now', '+1 day')) returning *
        "#,
        )        
        .bind(user.id)
        .bind(uuid::Uuid::new_v4().to_string())
        .fetch_one(&state.db_pool)
        .await
        .unwrap();

        let session_cookie = Cookie::new("session-token", new_session.token);

        _ = jar.add(session_cookie);

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

#[axum::debug_handler]
async fn sign_out_submit() -> Redirect {
    Redirect::to("/auth/sign-in")
}

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/sign-in", get(sign_in_page))
        .route("/sign-in", post(sign_in_submit))
        .route("/sign-up", get(sign_up_page))
        .route("/sign-up", post(sign_up_submit))
        .route("/sign-out", post(sign_out_submit).layer(middleware::from_extractor::<Authenticated>()))
}
