use ::goose::goose::*;
use async_trait::async_trait;
use goose_eggs::{validate_page, Validate};

use crate::responses::{ChallengeDTO, HttpResponseEnvelopeResult};

#[async_trait]
pub trait CheckResponse {
    async fn check_ok(self, user: &mut GooseUser, expected_status_code: u16) -> TransactionResult;
}

#[async_trait]
impl CheckResponse for GooseResponse {
    async fn check_ok(mut self, user: &mut GooseUser, expected_status_code: u16) -> TransactionResult {
        // let validate = Validate::builder().status(status_code).build();
        // validate_page(user, self, &validate).await?;

        let response = self.response.unwrap();

        let status_code = response.status();

        if (status_code.as_u16() != expected_status_code) {
            user.set_failure(
                &format!(
                    "{}: response status == {}]: {}",
                    self.request.raw.url, expected_status_code, status_code
                ),
                &mut self.request,
                None,
                None,
            )?;
        }

        let challenge_response = response
            .json::<HttpResponseEnvelopeResult<ChallengeDTO>>()
            .await
            .expect("Could not unwrap Challenge JSON.");

        println!("{}", challenge_response.result.id);

        Ok(())
    }
}
