use crate::helpers::config_builder::Builder;

#[derive(Clone, serde::Serialize)]
pub(crate) struct ServerConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<u16>,
}

pub(crate) struct ServerConfigBuilder {
    port: Option<u16>,
}

impl Default for ServerConfigBuilder {
    fn default() -> Self {
        ServerConfigBuilder { port: Some(0) }
    }
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

    fn build(self) -> ServerConfig {
        ServerConfig { port: self.port }
    }
}

impl Builder for ServerConfigBuilder {
    type Target = ServerConfig;

    fn build(self) -> Self::Target {
        self.build()
    }
}
