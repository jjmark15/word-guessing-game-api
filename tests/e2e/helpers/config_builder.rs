#[derive(serde::Serialize)]
pub(crate) struct ApplicationConfig {
    server: Option<ServerConfig>,
}

#[derive(Default)]
pub(crate) struct ApplicationConfigBuilder {
    server: Option<ServerConfig>,
}

impl ApplicationConfig {
    pub(crate) fn builder() -> ApplicationConfigBuilder {
        ApplicationConfigBuilder::default()
    }
}

impl ApplicationConfigBuilder {
    pub(crate) fn with_server(mut self, server: impl Into<Option<ServerConfig>>) -> Self {
        self.server = server.into();
        self
    }

    pub(crate) fn build(self) -> ApplicationConfig {
        ApplicationConfig {
            server: self.server,
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub(crate) struct ServerConfig {
    port: Option<u16>,
}

#[derive(Default)]
pub(crate) struct ServerConfigBuilder {
    port: Option<u16>,
}

impl ServerConfig {
    pub(crate) fn builder() -> ServerConfigBuilder {
        ServerConfigBuilder::default()
    }
}

impl ServerConfigBuilder {
    pub(crate) fn with_port(mut self, port: impl Into<Option<u16>>) -> Self {
        self.port = port.into();
        self
    }

    pub(crate) fn build(self) -> ServerConfig {
        ServerConfig { port: self.port }
    }
}
