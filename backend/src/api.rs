use axum:: {
    routing::get,
    routing::post,
    Router,
    extract::State,
    Json,
};

use serde::{Serialize, Deserialize};

use crate::AppState;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::scraper;

pub async fn get_routes(state : Arc<Mutex<AppState>>)-> axum::Router {
    Router::new()
        .route("/query", post(query))
        .route("/set-references", post(set_references))
        .route("/get-references", get(get_references))
        .with_state(state)

}

async fn get_references(state : State<Arc<Mutex<AppState>>>) -> Json<scraper::References> {
    Json(scraper::References {references: state.lock().await.get_references().await.clone() } )
}

async fn set_references(state : State<Arc<Mutex<AppState>>> , Json(payload) : Json<Reference>) {
    let mut app_state = state.lock().await;
    app_state.set_references(&payload.link).await;
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
struct Reference {
    link : String,
}


async fn query(state : State<Arc<Mutex<AppState>>> , Json(payload) : Json<Query>)-> Json<Response> {
    Json(Response { response :  state.lock().await.llm_query(&payload.query).await })
}

#[derive(Clone)]
#[derive(Deserialize)]
struct Query {
    query: String,
}

#[derive(Clone)]
#[derive(Serialize)]
struct Response {
    response : String,
}
