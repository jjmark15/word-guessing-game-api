use std::net::TcpListener;

use crate::http::axum_server::AxumServer;

mod http;

#[derive(derive_new::new)]
pub struct App {}

impl App {
    pub async fn run(&self, listener: TcpListener) {
        AxumServer::new().run(listener).await
    }
}
