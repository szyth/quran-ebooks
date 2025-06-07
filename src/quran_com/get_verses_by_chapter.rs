use std::collections::HashMap;

use crate::env;

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Data {
    pub(crate) verses: Vec<Verse>,
    pagination: Pagination,
}
#[derive(serde::Deserialize, Debug)]
pub(crate) struct Verse {
    id: u32,
    pub(crate) verse_number: u32,
    verse_key: String,
    hizb_number: u32,
    rub_el_hizb_number: u32,
    ruku_number: u32,
    manzil_number: u32,
    pub(crate) sajdah_number: Option<u32>,
    text_uthmani: String,
    pub(crate) text_indopak: String,
    pub(crate) page_number: u32,
    juz_number: u32,
    pub(crate) translations: Vec<Translation>,
    pub(crate) words: Vec<Word>,
}
#[derive(serde::Deserialize, Debug)]
pub(crate) struct Word {
    id: u32,
    position: u32,
    audio_url: Option<String>,
    char_type_name: String,
    verse_key: String,
    verse_id: u32,
    location: String,
    text_uthmani: String,
    text_indopak: String,
    text: String,
    page_number: u32,
    line_number: u32,
    pub(crate) translation: WordTranslation,
    transliteration: WordTransliteration,
}
#[derive(serde::Deserialize, Debug)]
pub(crate) struct WordTranslation {
    pub(crate) text: Option<String>,
    language_name: String,
}
#[derive(serde::Deserialize, Debug)]
pub(crate) struct WordTransliteration {
    text: Option<String>,
    language_name: String,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Translation {
    id: u32,
    resource_id: u32,
    pub(crate) text: String,
    verse_number: u32,
    page_number: u32,
}

#[derive(serde::Deserialize, Debug)]
struct Pagination {
    per_page: u32,
    current_page: u32,
    next_page: Option<u32>,
    total_pages: u32,
    total_records: u32,
}

// official docs:
// https://api-docs.quran.foundation/docs/content_apis_versioned/verses-by-chapter-number

pub(crate) async fn handler(
    surah_number: u8,
) -> Result<Data, Box<dyn std::error::Error + Sync + Send>> {
    let url = format!(
        "{}/verses/by_chapter/{}",
        env::api_url().unwrap(),
        surah_number
    ); // safe to use unwrap

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Referer", "https://quran.com".parse()?); // safe to use unwrap

    // headers.insert("x-auth-token", env::access_token().unwrap().parse()?); // safe to use unwrap
    // headers.insert("x-client-id", env::client_id().unwrap().parse()?); // safe to use unwrap

    let mut params = HashMap::new();
    params.insert("translations", "131");
    params.insert("translation_fields", "verse_number,page_number");
    params.insert("words", "true");
    params.insert(
        "word_fields",
        "verse_key,verse_id,page_number,location,text_uthmani,text_indopak,qpc_uthmani_hafs",
    );
    params.insert("fields", "text_uthmani,text_indopak");
    params.insert("per_page", "1000");

    let res = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .query(&params)
        .send()
        .await?
        .json::<Data>()
        .await;

    if res.is_err() {
        eprintln!("Error: Failed to fetch data from quran.com server.");
        eprintln!("{:#?}", res);
        std::process::exit(1)
    }

    Ok(res.unwrap()) // safe to use unwrap
}
