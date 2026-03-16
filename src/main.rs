use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::features::files::handler::file_page;

mod features;
mod shared;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut router = Router::new();

    // features
    router = router.route("/", get(file_page));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    axum::serve(listener, router).await?;

    Ok(())
}
