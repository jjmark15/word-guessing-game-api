use std::net::{IpAddr, Ipv6Addr, SocketAddr, TcpListener};

use wordle_api::App;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "wordle_api=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let address = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 3030);
    let listener = TcpListener::bind(address).unwrap();
    tracing::debug!("server listening on {}", address);

    let app = App::new();
    app.run(listener).await
}
