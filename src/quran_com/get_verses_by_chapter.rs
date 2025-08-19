use crate::{env, translations::verse};
use std::collections::HashMap;

// official docs:
// https://api-docs.quran.foundation/docs/content_apis_versioned/verses-by-chapter-number

pub(crate) async fn handler(
    surah_number: u8,
) -> Result<verse::Data, Box<dyn std::error::Error + Sync + Send>> {
    let url = format!(
        "{}/verses/by_chapter/{}",
        env::api_url().unwrap(),
        surah_number
    ); // safe to use unwrap

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Referer", "https://quran.com".parse()?); // safe to use unwrap

    headers.insert("x-auth-token", env::access_token().unwrap().parse()?); // safe to use unwrap
    headers.insert("x-client-id", env::client_id().unwrap().parse()?); // safe to use unwrap

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

    let res = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .query(&params)
        .send()
        .await?
        .json::<verse::Data>()
        .await;

    if res.is_err() {
        tracing::error!("Error: Failed to fetch data from quran.com server.");
        tracing::error!("{:#?}", res);
        std::process::exit(1)
    }

    Ok(res.unwrap()) // safe to use unwrap
}
