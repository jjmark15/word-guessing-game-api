use std::net::SocketAddr;

use reqwest::Response;

use crate::api_response::{ApiResponse, HttpResponseDetails};
use crate::guess_validation::GuessValidation;
use crate::response::{GuessValidationResponse, LatestChallengeResponse, ResponseBody};

#[derive(derive_new::new)]
pub struct Client {
    http_client: reqwest::Client,
    server_address: SocketAddr,
}

impl Client {
    pub async fn status(&self) -> ApiResponse<()> {
        let url = format!("http://{}/admin/status", self.server_address);
        let response = self.http_client.get(url).send().await.unwrap();
        ApiResponse::new((), Self::http_response_details(&response))
    }

    pub async fn validate(
        &self,
        challenge_id: impl AsRef<str>,
        guess: impl AsRef<str>,
    ) -> ApiResponse<GuessValidation> {
        let url = format!(
            "http://{}/challenge/{}/guess/validation/{}",
            self.server_address,
            challenge_id.as_ref(),
            guess.as_ref()
        );

        let response = self.http_client.get(url).send().await.unwrap();
        let response_details = Self::http_response_details(&response);

        let response_body = response
            .json::<ResponseBody<GuessValidationResponse>>()
            .await
            .unwrap();

        match response_body {
            ResponseBody::Error(error) => ApiResponse::from_error(error.into(), response_details),
            ResponseBody::Ok(validation_response) => {
                ApiResponse::new(validation_response.into(), response_details)
            }
        }
    }

    pub async fn latest_challenge(&self) -> ApiResponse<String> {
        let url = format!("http://{}/challenge/latest", self.server_address);

        let response = self.http_client.get(url).send().await.unwrap();
        let response_details = Self::http_response_details(&response);

        let response_body = response
            .json::<ResponseBody<LatestChallengeResponse>>()
            .await
            .unwrap();

        ApiResponse::new(response_body.unwrap().id().clone(), response_details)
    }

    fn http_response_details(response: &Response) -> HttpResponseDetails {
        HttpResponseDetails::new(response.status(), response.headers().clone())
    }
}
