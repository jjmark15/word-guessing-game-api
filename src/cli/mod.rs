use std::future::Future;
use std::net::SocketAddr;

use clap::Parser;

pub(crate) use opts::Opts;

use crate::{App, RunAppError};

mod opts;

pub async fn run_cli() -> Result<(impl Future<Output = ()>, SocketAddr), RunAppError> {
    let opts = Opts::parse();

    App::run(opts.config_path()).await
}
