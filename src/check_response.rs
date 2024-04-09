use ::goose::goose::*;
use async_trait::async_trait;
use goose_eggs::{validate_page, Validate};

#[async_trait]
pub trait CheckResponse {
    async fn check_ok(self, user: &mut GooseUser, status_code: u16) -> TransactionResult;
}

#[async_trait]
impl CheckResponse for GooseResponse {
    async fn check_ok(self, user: &mut GooseUser, status_code: u16) -> TransactionResult {
        let validate = Validate::builder().status(status_code).build();

        validate_page(user, self, &validate).await?;

        Ok(())
    }
}
