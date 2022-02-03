use std::net::{IpAddr, Ipv6Addr, SocketAddr, TcpListener};

use wordle_api::App;

#[tokio::main]
async fn main() {
    let address = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 3030);
    let listener = TcpListener::bind(address).unwrap();
    println!("listening on {}", address);

    let app = App::new();
    app.run(listener).await
}
