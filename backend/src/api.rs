use axum:: {
    routing::get,
    routing::post,
    Router,
    extract::State,
    Json,
};

use serde::Deserialize;

use crate::AppState;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::scraper;
use http::Method;
use tower_http::cors::{Any, CorsLayer};
use crate::openai;

pub async fn get_routes(state : Arc<Mutex<AppState>>)-> axum::Router {

     let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    Router::new()
        .route("/query", post(query))
        .route("/set-references", post(set_references))
        .route("/get-references", get(get_references))
        .route("/get-main-reference", get(get_main_reference))
        .route("/reset-llm", post(reset))
        .with_state(state)
        .layer(cors)

}

async fn get_main_reference(state : State<Arc<Mutex<AppState>>>) -> Json<scraper::Reference> {
    Json(state.lock().await.get_main_reference().await.expect("expected a inputted wikipedia site"))
}

async fn get_references(state : State<Arc<Mutex<AppState>>>) -> Json<scraper::References> {
    Json(scraper::References {references: state.lock().await.get_references().await.clone() } )
}

async fn set_references(state : State<Arc<Mutex<AppState>>> , Json(payload) : Json<scraper::Reference>) {
    let mut app_state = state.lock().await;
    app_state.set_references(&payload.link).await;
}

async fn query(state : State<Arc<Mutex<AppState>>>, Json(payload) : Json<Query>) -> Json<openai::QueryResult> {
    Json(state.lock().await.llm_query(&payload.query).await)
}

async fn reset(state : State<Arc<Mutex<AppState>>>) {
    state.lock().await.reload_llmclient(crate::LlmOptions::BASE).await;
}

#[derive(Clone)]
#[derive(Deserialize)]
struct Query {
    query: String,
}
