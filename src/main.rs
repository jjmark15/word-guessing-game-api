use std::process::exit;

use word_guessing_game_api::cli::run_cli;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "word_guessing_game_api=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    match run_cli().await {
        Ok((future, _)) => future.await,
        Err(error) => {
            tracing::error!("{error}");
            exit(1);
        }
    }
}
