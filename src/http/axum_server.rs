use std::net::TcpListener;
use std::sync::Arc;

use axum::extract::Extension;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::application::ApplicationService;
use crate::http::latest_challenge::latest_challenge_handler;
use crate::http::status::status_handler;
use crate::http::validate::validation_handler;

pub(crate) struct AxumServer {}

impl AxumServer {
    pub async fn run(listener: TcpListener, application_service: ApplicationService) {
        let application_service = Arc::new(application_service);

        let app = Router::new()
            .route("/admin/status", get(status_handler))
            .route("/challenge/latest", get(latest_challenge_handler))
            .route(
                "/challenge/:challenge_id/guess/validation/:guess",
                get(validation_handler),
            )
            .layer(Extension(application_service.clone()))
            .layer(TraceLayer::new_for_http());

        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
