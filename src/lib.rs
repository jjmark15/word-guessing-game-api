extern crate core;

use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::path::Path;

use crate::application::ApplicationService;
use crate::config::{
    ApplicationConfig, ConfigReader, ConfigValidator, ReadConfigError, ValidationError,
};
use crate::domain::{AnswerRepository, GuessValidator};
use crate::http::axum_server::AxumServer;

mod application;
pub mod cli;
mod config;
mod domain;
mod http;

pub struct App {}

impl App {
    pub async fn run(
        config_path: &Path,
    ) -> Result<(impl Future<Output = ()>, SocketAddr), RunAppError> {
        let config: ApplicationConfig = ConfigReader::new()
            .read(config_path)
            .map_err(RunAppError::from)?;

        ConfigValidator::new()
            .validate(&config)
            .map_err(RunAppError::from)?;

        let listener = Self::tcp_listener(*config.server().port());
        let socket_address = listener.local_addr().unwrap();
        tracing::info!("server listening on {socket_address}");

        Ok((
            AxumServer::run(listener, Self::application_service(&config)),
            socket_address,
        ))
    }

    fn tcp_listener(port: u16) -> TcpListener {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
        TcpListener::bind(address).unwrap()
    }

    fn application_service(config: &ApplicationConfig) -> ApplicationService {
        let answer_repository = AnswerRepository::new(config.challenges().answers().clone());
        ApplicationService::new(GuessValidator::new(), answer_repository)
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct RunAppError {
    source: RunAppErrorKind,
}

impl RunAppError {
    fn from<E: Into<RunAppErrorKind>>(source: E) -> Self {
        RunAppError {
            source: source.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
enum RunAppErrorKind {
    ReadConfig(#[from] ReadConfigError),
    ValidateConfig(#[from] ValidationError),
}
