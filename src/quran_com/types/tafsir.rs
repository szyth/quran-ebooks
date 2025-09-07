use crate::quran_com::apis::get_tafsir_for_surah;

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Data {
    pub(crate) tafsirs: Vec<Tafsir>,
    pagination: Pagination,
}
#[derive(serde::Deserialize, Debug)]
struct Pagination {
    per_page: u32,
    current_page: u32,
    next_page: Option<u32>,
    total_pages: u32,
    total_records: u32,
}
#[derive(serde::Deserialize, Debug)]
pub(crate) struct Tafsir {
    id: usize,
    resource_id: usize,
    verse_key: String,
    language_id: usize,
    pub(crate) text: String,
}
#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("JSONParseError: {0}")]
    JSONParseError(#[from] serde_json::Error),
    #[error("GetTafsirForSurahError: {0}")]
    GetTafsirForSurahError(#[from] get_tafsir_for_surah::Error),
}
impl Tafsir {
    pub(crate) async fn by_surah(surah_number: u8, resource_id: usize) -> Result<Data, Error> {
        tracing::info!("Fetching Verses from quran.com server");
        let res = crate::quran_com::apis::get_tafsir_for_surah::handler(surah_number, resource_id)
            .await?;
        let data: Data = serde_json::from_value(res)?;
        Ok(data)
    }
}
