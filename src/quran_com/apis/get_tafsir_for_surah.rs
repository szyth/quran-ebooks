use reqwest::header::HeaderMap;

use crate::{
    env,
    utils::http::{ACCESS_TOKEN, HTTP_CLIENT},
};
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("JSONParseError: {0}")]
    JSONParseError(#[from] serde_json::Error),
    #[error("Header parse error: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
    #[error("ReqWestError: {0}")]
    ReqWestError(#[from] reqwest::Error),
}

// official docs:
// https://api-docs.quran.foundation/docs/content_apis_versioned/verses-by-chapter-number

pub(crate) async fn handler(
    surah_number: u8,
    resource_id: usize,
) -> Result<serde_json::Value, Error> {
    let url = format!(
        "{}/tafsirs/{}/by_chapter/{}",
        env::api_url().unwrap(),
        resource_id,
        surah_number
    ); // safe to use unwrap

    let client = HTTP_CLIENT.get().expect("HTTP_CLIENT not initialized");
    let token = ACCESS_TOKEN.get().expect("ACCESS_TOKEN not initialized");
    let headers = {
        let mut h = HeaderMap::new();
        h.insert("Referer", "https://quran.com".parse()?);
        h.insert("x-auth-token", token.parse()?); // safe to use unwrap
        h.insert("x-client-id", env::client_id().unwrap().parse()?); // safe to use unwrap
        h
    };

    let mut params = HashMap::new();
    params.insert("per_page", "1000");

    let body: serde_json::Value = client
        .get(url)
        .headers(headers)
        .query(&params)
        .send()
        .await?
        .json()
        .await?;

    Ok(body)
}
