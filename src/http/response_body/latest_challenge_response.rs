use serde::Serialize;

#[derive(Serialize, derive_new::new)]
pub(crate) struct LatestChallengeResponse {
    id: String,
}
