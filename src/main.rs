use ::goose::{config::*, goose::*, *};
use async_trait::async_trait;
use goose_eggs::{validate_page, Validate};
use std::{env, time::*};

#[async_trait]
trait CheckResponse {
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

async fn challenges_test(user: &mut GooseUser) -> TransactionResult {
    user.post("/api/v1/Challenges", "")
        .await?
        .check_ok(user, 201)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let args: Vec<String> = env::args().collect();
    let host = &args[1];
    let mut config = GooseConfiguration::default();

    // apply common options
    config.host = host.to_string();
    config.no_reset_metrics = true;
    config.report_file = format!("reports/report-{}.html", current_timestamp);
    config.run_time = String::from("60s");

    GooseAttack::initialize_with_config(config)?
        .register_scenario(
            scenario!("LoadtestTransactions").register_transaction(transaction!(challenges_test)),
        )
        .execute()
        .await?;

    Ok(())
}
