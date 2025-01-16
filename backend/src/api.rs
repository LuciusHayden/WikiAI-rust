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

pub async fn get_routes(state : Arc<AppState>)-> axum::Router {
    Router::new()
        .route("/query", post(query))
        .with_state(state)

}

async fn query(state : State<Arc<AppState>> , Json(payload) : Json<Query>)-> Json<Response> {
    Json(Response { response :  state.llm_query(&payload.query).await })
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
