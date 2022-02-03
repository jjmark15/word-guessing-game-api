use std::net::TcpListener;
use std::sync::Arc;

use axum::{routing::get, AddExtensionLayer, Router};
use tower_http::trace::TraceLayer;

use crate::application::ApplicationService;
use crate::http::status::status_handler;
use crate::http::validate::validation_handler;

#[derive(derive_new::new)]
pub(crate) struct AxumServer {}

impl AxumServer {
    pub async fn run(&self, listener: TcpListener, application_service: ApplicationService) {
        let application_service = Arc::new(application_service);

        let app = Router::new()
            .route("/admin/status", get(status_handler))
            .route("/guess/validate/:guess", get(validation_handler))
            .layer(AddExtensionLayer::new(application_service.clone()))
            .layer(TraceLayer::new_for_http());

        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
