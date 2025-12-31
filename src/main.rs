use axum::Router;
use dotenv::dotenv;
pub mod audio;
pub mod error;
mod llm;
mod router;
mod templates;
mod zip;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app = Router::new().merge(router::router());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
