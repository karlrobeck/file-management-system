use axum::{Router, routing::get};
use axum_extra::extract::cookie::Key;
use tokio::net::TcpListener;

use crate::features::{
    auth,
    files::{self, handler::file_page},
};

mod features;
mod shared;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut router = Router::new();

    let context = {
        let db_pool = sqlx::SqlitePool::connect(env!("DATABASE_URL")).await?;

        // migrate
        sqlx::migrate!("./migrations").run(&db_pool).await?;

        shared::context::AppContext {
            db_pool,
            key: Key::generate(),
        }
    };

    // features
    router = router
        .merge(files::handler::router(&context))
        .nest("/auth", auth::handler::router(&context));

    let router = router.with_state(context);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    axum::serve(listener, router).await?;

    Ok(())
}
