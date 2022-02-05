use serde::Deserialize;

#[derive(Deserialize, derive_getters::Getters)]
pub(crate) struct LatestChallengeResponse {
    id: String,
}
