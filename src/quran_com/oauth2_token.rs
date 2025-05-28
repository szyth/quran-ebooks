use crate::env;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct LoginResponse {
    access_token: String,
    expires_in: u32,
    scope: String,
    token_type: String,
}

// official docs:
// https://api-docs.quran.foundation/docs/oauth2_apis_versioned/oauth-2-token-exchange

pub(crate) async fn handler() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/oauth2/token", env::auth_url().unwrap()); // safe to use unwrap
    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("scope", "content");

    let res = reqwest::Client::new()
        .post(url)
        .basic_auth(
            env::client_id().unwrap(),           // safe to use unwrap
            Some(env::client_secret().unwrap()), // safe to use unwrap
        )
        .form(&params) // set Content-Type: application/x-www-form-urlencoded
        .send()
        .await?
        .json::<LoginResponse>()
        .await;

    if res.is_err() {
        eprintln!("Error: Login failed");
        std::process::exit(1)
    }

    println!(
        "Please update .env with this Access Token: \n\n{}",
        res.unwrap().access_token // safe to use unwrap
    );

    Ok(())
}
