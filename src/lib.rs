use std::net::TcpListener;

use crate::application::ApplicationService;
use crate::domain::GuessValidator;
use crate::http::axum_server::AxumServer;

mod application;
mod domain;
mod http;

#[derive(derive_new::new)]
pub struct App {}

impl App {
    pub async fn run(&self, listener: TcpListener) {
        AxumServer::new()
            .run(listener, ApplicationService::new(GuessValidator::new()))
            .await
    }
}
