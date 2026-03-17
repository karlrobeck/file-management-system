use axum::extract::{FromRef, FromRequestParts};
use axum_extra::extract::cookie::Key;

#[derive(Clone)]
pub struct AppContext {
    pub db_pool: sqlx::SqlitePool,
    pub key: Key,
}

impl<S> FromRequestParts<S> for AppContext
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}

impl FromRef<AppContext> for Key {
    fn from_ref(input: &AppContext) -> Self {
        input.key.clone()
    }
}
