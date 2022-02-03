use std::net::TcpListener;

use axum::http::StatusCode;
use axum::{routing::get, Router};

#[derive(derive_new::new)]
pub struct App {}

impl App {
    pub async fn run(&self, listener: TcpListener) {
        let app = Router::new().route("/admin/status", get(handler));

        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

async fn handler() -> StatusCode {
    StatusCode::OK
}
