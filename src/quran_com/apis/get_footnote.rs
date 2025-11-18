use regex::Regex;
use reqwest::header::HeaderMap;

use crate::{
    env,
    utils::http::{ACCESS_TOKEN, HTTP_CLIENT},
};

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("JSONParseError: {0}")]
    JSONParseError(#[from] serde_json::Error),
    #[error("ReqWestError: {0}")]
    ReqWestError(#[from] reqwest::Error),
    #[error("RegexError: {0}")]
    RegexError(#[from] regex::Error),
    #[error("Header parse error: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
}

#[derive(serde::Deserialize, Debug)]
struct Response {
    foot_note: Footnote,
}

#[derive(serde::Deserialize, Debug)]
struct Footnote {
    id: u32,
    text: String,
    language_name: String,
}
pub(crate) async fn handler(text: &str) -> Result<Vec<String>, Error> {
    let mut footnote: Vec<String> = vec![];
    let client = HTTP_CLIENT.get().expect("HTTP_CLIENT not initialized");
    let token = ACCESS_TOKEN.get().expect("ACCESS_TOKEN not initialized");
    let headers = {
        let mut h = HeaderMap::new();
        h.insert("Referer", "https://quran.com".parse()?);
        h.insert("x-auth-token", token.parse()?); // safe to use unwrap
        h.insert("x-client-id", env::client_id().unwrap().parse()?); // safe to use unwrap
        h
    };

    let re = Regex::new(r#"<sup\s+foot_note=(?:"|')?(?P<footnote>\d+)(?:"|')?>"#).unwrap();
    for caps in re.captures_iter(&text) {
        let url = format!(
            "{}/foot_notes/{}",
            env::api_url().unwrap(),
            &caps["footnote"]
        ); // safe to use unwrap
        tracing::info!("Fetching footnotes on: {}", url);

        let body: String = client
            .get(&url)
            .headers(headers.clone())
            .send()
            .await?
            .text()
            .await?;

        let parsed: Response = serde_json::from_str(&body.clone()).map_err(|e| {
            tracing::error!("Raw quran.com API response: {}", body);
            e
        })?;

        footnote.push(parsed.foot_note.text);
    }

    Ok(footnote)
}
