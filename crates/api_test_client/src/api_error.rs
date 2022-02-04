#[derive(Debug, derive_getters::Getters, derive_new::new)]
pub struct ApiError {
    message: String,
}
