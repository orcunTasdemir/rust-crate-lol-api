use std::net::SocketAddr;

use crate::model::Champions::ModelController;

pub use self::error::{Error, Result};
use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[tokio::main]
async fn main() -> Result<()> {
    //Initialize the model controller
    let mc = ModelController::new().await?;

    let routes_all = Router::new()
        .nest("/api", web::routes_champions::routes(mc.clone()))
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
