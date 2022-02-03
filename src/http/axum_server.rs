use std::net::TcpListener;

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::http::status::status_handler;

#[derive(derive_new::new)]
pub(crate) struct AxumServer {}

impl AxumServer {
    pub async fn run(&self, listener: TcpListener) {
        let app = Router::new()
            .route("/admin/status", get(status_handler))
            .layer(TraceLayer::new_for_http());

        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}