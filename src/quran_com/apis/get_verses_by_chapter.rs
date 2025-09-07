use reqwest::header::HeaderMap;

use crate::{
    env,
    quran_com::types::verse,
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

pub(crate) async fn handler(surah_number: u8) -> Result<verse::VerseData, Error> {
    let url = format!(
        "{}/verses/by_chapter/{}",
        env::api_url().unwrap(),
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
    params.insert("translations", "20");
    params.insert("translation_fields", "verse_number,page_number");
    params.insert("words", "true");
    params.insert(
        "word_fields",
        "verse_key,verse_id,page_number,location,text_uthmani,text_indopak_nastaleeq,qpc_uthmani_hafs",
    );
    params.insert("fields", "text_uthmani,text_indopak_nastaleeq");
    params.insert("per_page", "1000");

    let body = client
        .get(url)
        .headers(headers)
        .query(&params)
        .send()
        .await?
        .text()
        .await?;

    let parsed: verse::VerseData = serde_json::from_str(&body.clone()).map_err(|e| {
        tracing::error!("Raw quran.com API response: {}", body);
        e
    })?;

    Ok(parsed)
}
