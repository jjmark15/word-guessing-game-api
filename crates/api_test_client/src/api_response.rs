use std::fmt::Debug;

use http::header::HeaderName;
use http::StatusCode;

pub struct ApiResponse<T, E> {
    value: Result<T, E>,
    http_response_details: HttpResponseDetails,
}

impl<T, E: Debug> ApiResponse<T, E> {
    pub fn new(value: T, http_response_details: HttpResponseDetails) -> Self {
        ApiResponse {
            value: Ok(value),
            http_response_details,
        }
    }

    pub fn from_error(error: E, http_response_details: HttpResponseDetails) -> Self {
        ApiResponse {
            value: Err(error),
            http_response_details,
        }
    }

    pub fn value(self) -> T {
        self.value.unwrap()
    }

    pub fn error(self) -> E {
        self.value.err().unwrap()
    }

    pub fn http_response_details(&self) -> &HttpResponseDetails {
        &self.http_response_details
    }
}

#[derive(derive_new::new, derive_getters::Getters)]
pub struct HttpResponseDetails {
    status_code: StatusCode,
    headers: http::HeaderMap,
}

impl HttpResponseDetails {
    pub fn header_value(&self, header_name: HeaderName) -> Option<String> {
        self.headers
            .get(header_name)
            .map(|value| value.to_str().unwrap().to_string())
    }
}
