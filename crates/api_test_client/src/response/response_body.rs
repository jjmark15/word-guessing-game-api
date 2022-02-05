use serde::Deserialize;

use crate::response::ErrorResponse;

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum ResponseBody<T> {
    Error(ErrorResponse),
    Ok(T),
}
