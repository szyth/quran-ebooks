use crate::{env, utils::http::HTTP_CLIENT};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct LoginResponse {
    access_token: String,
    expires_in: u32,
    scope: String,
    token_type: String,
}
#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Header parse error: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
    #[error("ReqWestError: {0}")]
    ReqWestError(#[from] reqwest::Error),
    #[error("JSONParseError: {0}")]
    JSONParseError(#[from] serde_json::Error),
}
// official docs:
// https://api-docs.quran.foundation/docs/oauth2_apis_versioned/oauth-2-token-exchange

pub(crate) async fn handler() -> Result<String, Error> {
    let url = format!("{}/oauth2/token", env::auth_url().unwrap()); // safe to use unwrap
    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("scope", "content");

    let client = HTTP_CLIENT.get().expect("HTTP_CLIENT not initialized");
    let body = client
        .post(url)
        .basic_auth(
            env::client_id().unwrap(),           // safe to use unwrap
            Some(env::client_secret().unwrap()), // safe to use unwrap
        )
        .form(&params) // set Content-Type: application/x-www-form-urlencoded
        .send()
        .await?
        .text()
        .await?;

    let parsed: LoginResponse = serde_json::from_str(&body.clone()).map_err(|e| {
        tracing::error!("Raw quran.com API response: {}", body);
        e
    })?;

    tracing::info!("Login Successful. Access Token: \n{}", parsed.access_token);

    Ok(parsed.access_token)
}
