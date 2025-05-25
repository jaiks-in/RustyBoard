use axum::{
    Router,http::Method
};
use tower_http::cors::{CorsLayer,Any};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing_subscriber;
use db::connect_db;
use routes::task_router;
use crate::routes::task_router::task_route;

mod db;
mod routes;
#[tokio::main]
async fn main(){
    dotenv().ok();
    let cors=CorsLayer::new().allow_methods(vec![Method::GET]).allow_origin(Any);
    let app=Router::new().merge(task_router::task_route()).layer(cors);
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}
// #[tokio::main]
// async fn main() {
//     // Load env variables
//     dotenv().ok();
//     tracing_subscriber::fmt::init();
//
//     // Connect to database
//     let _pool = connect_db().await;
//
//     // CORS configuration
//     let cors = CorsLayer::new()
//         .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
//         .allow_origin(Any);
//
//     // Application router with routes and CORS layer
//     let app = Router::new()
//         .merge(task_router::task_route())
//         .layer(cors);
//
//     // TCP Listener on port 8080
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:8085").await.unwrap();
//
//     tracing::info!("🚀 Server running on {}", listener.local_addr().unwrap());
//
//     // Serve application with listener
//     axum::serve(listener, app).await.unwrap();
// }
