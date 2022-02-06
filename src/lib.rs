extern crate core;

use std::future::Future;
use std::net::{IpAddr, Ipv6Addr, SocketAddr, TcpListener};
use std::path::Path;

use crate::application::ApplicationService;
use crate::config::{ApplicationConfig, ConfigReader, ReadConfigError};
use crate::domain::GuessValidator;
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

        let listener = Self::tcp_listener(*config.server().port());
        let socket_address = listener.local_addr().unwrap();
        tracing::info!("server listening on {socket_address}");

        Ok((
            AxumServer::run(listener, ApplicationService::new(GuessValidator::new())),
            socket_address,
        ))
    }

    fn tcp_listener(port: u16) -> TcpListener {
        let address = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), port);
        TcpListener::bind(address).unwrap()
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
}
