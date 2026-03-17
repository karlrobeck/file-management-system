use axum::extract::{FromRef, FromRequest};
use axum_extra::extract::cookie::Key;

#[derive(Clone)]
pub struct AppContext {
    pub db_pool: sqlx::SqlitePool,
    pub key: Key,
}

impl<S> FromRequest<S> for AppContext
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request(
        mut req: axum::extract::Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let state = AppContext::from_ref(state);

        req.extensions_mut().insert(state.clone());

        Ok(state)
    }
}

impl FromRef<AppContext> for Key {
    fn from_ref(input: &AppContext) -> Self {
        input.key.clone()
    }
}
