use serde::Deserialize;

use crate::response::ErrorResponse;

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum ResponseBody<T> {
    Error(ErrorResponse),
    Ok(T),
}

impl<T> ResponseBody<T> {
    pub(crate) fn unwrap(self) -> T {
        if let ResponseBody::Ok(inner) = self {
            inner
        } else {
            panic!("response body was not Ok")
        }
    }
}
