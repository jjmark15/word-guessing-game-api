#[derive(serde::Serialize)]
pub(crate) struct ApplicationConfig {
    server: Option<ServerConfig>,
}

pub(crate) struct ApplicationConfigBuilder {
    server: Option<ServerConfig>,
    challenges: Option<ChallengesConfig>,
}

impl Default for ApplicationConfigBuilder {
    fn default() -> Self {
        ApplicationConfigBuilder {
            server: Some(ServerConfigBuilder::default().build()),
            challenges: Some(ChallengesConfigBuilder::default().build()),
        }
    }
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

    pub(crate) fn with_challenges(
        mut self,
        challenges: impl Into<Option<ChallengesConfig>>,
    ) -> Self {
        self.challenges = challenges.into();
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

    pub(crate) fn build(self) -> ServerConfig {
        ServerConfig { port: self.port }
    }
}

#[derive(Clone, serde::Serialize)]
pub(crate) struct ChallengesConfig {
    answers: Option<Vec<String>>,
}

impl ChallengesConfig {
    pub(crate) fn builder() -> ChallengesConfigBuilder {
        ChallengesConfigBuilder::default()
    }
}

pub(crate) struct ChallengesConfigBuilder {
    answers: Option<Vec<String>>,
}

impl Default for ChallengesConfigBuilder {
    fn default() -> Self {
        ChallengesConfigBuilder {
            answers: Some(vec![]),
        }
    }
}

impl ChallengesConfigBuilder {
    pub(crate) fn with_answers(mut self, answers: impl Into<Option<Vec<String>>>) -> Self {
        self.answers = answers.into();
        self
    }

    pub(crate) fn build(self) -> ChallengesConfig {
        ChallengesConfig {
            answers: self.answers,
        }
    }
}
