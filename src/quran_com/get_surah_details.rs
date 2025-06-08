use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct SurahDetails {
    id: u8,
    #[serde(rename = "transliteratedName")]
    pub(crate) transliterated_name: String,
    #[serde(rename = "revelationPlace")]
    revelation_place: String,
    #[serde(rename = "versesCount")]
    verses_count: u32,
    #[serde(rename = "translatedName")]
    translated_name: String,
    slug: String,
}
pub(crate) fn handler(
    surah_number: u8,
) -> Result<SurahDetails, Box<dyn std::error::Error + Sync + Send>> {
    let list_raw = std::fs::read_to_string("static/quran_surahs.json")?;
    let list: Vec<SurahDetails> = serde_json::from_str(&list_raw)?;

    let index = list.get(surah_number as usize - 1);

    if index.is_none() {
        eprintln!("Error: Failed to fetch surah details from `static/quran_surahs.json`");
        std::process::exit(1)
    }
    let index = index.unwrap(); // safe to use unwrap()

    if index.id != surah_number {
        eprintln!("Error: Invalid data in `static/quran_surahs.json`");
        std::process::exit(1)
    }

    return Ok(index.clone());
}
