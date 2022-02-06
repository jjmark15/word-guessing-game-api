use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use api_test_client::{Client, HttpClient};

pub(crate) struct TestContext {
    client: Client,
}

impl TestContext {
    pub(crate) fn new() -> Self {
        let local_address = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 3030);

        let client = Client::new(HttpClient::new(), local_address);
        TestContext { client }
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }
}
