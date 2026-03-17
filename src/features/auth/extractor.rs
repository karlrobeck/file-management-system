use axum::{
    Extension, RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    response::Redirect,
};
use axum_extra::extract::{PrivateCookieJar, cookie::Key};

use crate::{features::auth::model::Session, shared::context::AppContext};

impl<S> FromRequestParts<S> for Session
where
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Redirect;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let state = AppContext::from_ref(state);
        let private_jar = PrivateCookieJar::<Key>::from_request_parts(parts, &state)
            .await
            .unwrap();

        let session_token = match private_jar.get("session-token") {
            Some(cookie) => cookie.value().to_string(),
            None => return Err(Redirect::to("/auth/sign-in")),
        };

        let session = sqlx::query_as::<_, Session>(
            r#"
            select * from sessions where token = ? and expires_at > current_timestamp
        "#,
        )
        .bind(session_token)
        .fetch_optional(&state.db_pool)
        .await
        .unwrap();

        if let Some(session) = session {
            Ok(session)
        } else {
            Err(Redirect::to("/auth/sign-in"))
        }
    }
}

pub struct Authenticated;

impl<S> FromRequestParts<S> for Authenticated
where
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Redirect;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let _session = Session::from_request_parts(parts, state).await?;
        Ok(Authenticated)
    }
}
