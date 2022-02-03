use std::net::SocketAddr;

use axum::http::StatusCode;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/admin/status", get(handler));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> StatusCode {
    StatusCode::OK
}
