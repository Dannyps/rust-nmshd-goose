use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChallengeDTO {
    pub id: String,
    pub expiresAt: String,
    pub createdBy: Option<String>,
    pub createdByDevice: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct HttpResponseEnvelopeResult<T> {
    pub result: T,
}
