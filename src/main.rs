use axum::Router;
use dotenv::dotenv;
mod llm;
mod llm2;
mod router;
mod templates;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // let app = Router::new().merge(router::router());
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    // axum::serve(listener, app).await?;

    llm2::send_message().await?;

    Ok(())
}
