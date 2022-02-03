use std::net::SocketAddr;

use http::StatusCode;

#[derive(derive_new::new)]
pub struct Client {
    http_client: reqwest::Client,
    server_address: SocketAddr,
}

impl Client {
    pub async fn status(&self) -> StatusCode {
        let url = format!("http://{}/admin/status", self.server_address);
        self.http_client.get(url).send().await.unwrap().status()
    }
}
