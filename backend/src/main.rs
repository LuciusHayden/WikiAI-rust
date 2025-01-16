pub mod api;
pub mod openai;
pub mod scraper;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use api::*;
use backend::AppState;

use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    
    let app_state = AppState::new("https://en.wikipedia.org/wiki/Chocolate_chip_cookie", backend::openai::LlmOptions::RAG).await;
    let state = Arc::new(app_state);
    
    let app = api::get_routes(state).await; 
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}




