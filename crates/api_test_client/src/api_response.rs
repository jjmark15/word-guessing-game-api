use http::header::HeaderName;
use http::StatusCode;

#[derive(derive_new::new)]
pub struct ApiResponse<T> {
    value: T,
    http_response_details: HttpResponseDetails,
}

impl<T> ApiResponse<T> {
    pub fn value(self) -> T {
        self.value
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
