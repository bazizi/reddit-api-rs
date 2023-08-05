use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthParams {
    pub state: String,
    pub code: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct AuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
    pub refresh_token: String,
}
