use axum::{
    Router,routing::{get,post},Json
};
use serde_json::{Value as JsonValue,json};
pub async fn health_check()->Json<JsonValue>{
    Json(json!({ "status": "Healthy",
   "service": "RustyBoard Backend"}))
}
pub fn task_route()->Router{
    Router::new().route("/health",get(health_check))
}

// use axum::{Router, routing::get, Json};
// use serde_json::json;
// use serde_json::Value as JsonValue;
//
// pub async fn health_check() -> Json<JsonValue> {
//     Json(json!({
//         "status": "Healthy",
//         "service": "RustyBoard Backend"
//     }))
// }
//
// pub fn task_route() -> Router {
//     Router::new().route("/health", get(health_check))
// }
