use crate::env;
use std::collections::HashMap;

// official docs:
// https://api-docs.quran.foundation/docs/content_apis_versioned/verses-by-chapter-number

pub(crate) async fn handler(
    surah_number: u8,
    resource_id: usize,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Sync + Send>> {
    let url = format!(
        "{}/tafsirs/{}/by_chapter/{}",
        env::api_url().unwrap(),
        resource_id,
        surah_number
    ); // safe to use unwrap

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Referer", "https://quran.com".parse()?); // safe to use unwrap

    // headers.insert("x-auth-token", env::access_token().unwrap().parse()?); // safe to use unwrap
    // headers.insert("x-client-id", env::client_id().unwrap().parse()?); // safe to use unwrap

    let mut params = HashMap::new();
    params.insert("per_page", "1000");

    let res = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .query(&params)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await;

    if res.is_err() {
        tracing::error!("Error: Failed to fetch data from quran.com server.");
        tracing::error!("{:#?}", res);
        std::process::exit(1)
    }

    Ok(res.unwrap()) // safe to use unwrap
}
