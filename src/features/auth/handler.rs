use askama::Template;
use axum::{
    Form, Router,
    extract::State,
    middleware,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use axum_extra::extract::{PrivateCookieJar, cookie::Cookie};

use crate::{
    features::auth::{
        extractor::Authenticated,
        model::{Session, User},
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
async fn sign_in_page(_: State<AppContext>, jar: PrivateCookieJar) -> impl IntoResponse {
    if jar.get("session-token").is_some() {
        return (jar, Redirect::to("/").into_response());
    }
    (jar, Html(SignInPage.render().unwrap()).into_response())
}

#[axum::debug_handler]
async fn sign_in_submit(
    State(state): State<AppContext>,
    jar: PrivateCookieJar,
    Form(payload): Form<SignInFormRequest>,
) -> (PrivateCookieJar, Redirect) {
    if jar.get("session-token").is_some() {
        return (jar, Redirect::to("/"));
    }

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

        let session_cookie = Cookie::build(("session-token", new_session.token.clone()))
            .path("/")
            .secure(true)
            .http_only(true);

        let updated_jar = jar.add(session_cookie);

        println!("New Session: {:?}", new_session);

        return (updated_jar, Redirect::to("/"));
    }

    (jar, Redirect::to("/auth/sign-in"))
}

#[axum::debug_handler]
async fn sign_up_page(_: State<AppContext>, jar: PrivateCookieJar) -> impl IntoResponse {
    if jar.get("session-token").is_some() {
        return (jar, Redirect::to("/").into_response());
    }

    (jar, Html(SignUpPage.render().unwrap()).into_response())
}

#[axum::debug_handler]
async fn sign_up_submit(
    State(state): State<AppContext>,
    jar: PrivateCookieJar,
    Form(payload): Form<SignUpFormRequest>,
) -> impl IntoResponse {
    if jar.get("session-token").is_some() {
        return (jar, Redirect::to("/").into_response());
    }

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

    trx.commit().await.unwrap();

    (jar, Redirect::to("/").into_response())
}

#[axum::debug_handler]
async fn sign_out_submit(_: State<AppContext>, jar: PrivateCookieJar) -> impl IntoResponse {
    let updated_jar = jar.remove(Cookie::build("session-token").path("/").finish());
    (updated_jar, Redirect::to("/auth/sign-in").into_response())
}

pub fn router(state: &AppContext) -> Router<AppContext> {
    Router::new()
        .route("/sign-in", get(sign_in_page))
        .route("/sign-in", post(sign_in_submit))
        .route("/sign-up", get(sign_up_page))
        .route("/sign-up", post(sign_up_submit))
        .route(
            "/sign-out",
            post(sign_out_submit).layer(middleware::from_extractor_with_state::<Authenticated, _>(
                state.clone(),
            )),
        )
}
