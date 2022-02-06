use std::net::{IpAddr, Ipv6Addr, SocketAddr, TcpListener};

use tokio::task::JoinHandle;

use api_test_client::{Client, HttpClient};
use word_guessing_game_api::App;

pub(crate) struct TestContext {
    client: Client,
}

impl TestContext {
    pub(crate) fn new() -> Self {
        let listener = Self::tcp_listener();
        let local_address = listener.local_addr().unwrap();

        let _app = Self::app(listener);

        let client = Client::new(HttpClient::new(), local_address);
        TestContext { client }
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }

    fn tcp_listener() -> TcpListener {
        TcpListener::bind(SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 0)).unwrap()
    }

    fn app(listener: TcpListener) -> JoinHandle<()> {
        let app = App::new();
        tokio::spawn(async move { app.run(listener).await })
    }
}
