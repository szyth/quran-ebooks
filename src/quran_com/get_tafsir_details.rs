use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct TafsirDetails {
    id: u8,
    resource_id: usize,
    resource_name: String,
    pub(crate) translated_name: String,
    slug: String,
}
#[tracing::instrument(skip_all)]
pub(crate) fn handler(
    resource_id: usize,
) -> Result<TafsirDetails, Box<dyn std::error::Error + Sync + Send>> {
    let list_raw = std::fs::read_to_string("static/tafsirs.json")?;
    let list: Vec<TafsirDetails> = serde_json::from_str(&list_raw)?;

    let tafsir_index = list
        .iter()
        .position(|tafsir| tafsir.resource_id == resource_id);

    if tafsir_index.is_none() {
        tracing::error!("Error: Failed to fetch tafsir details from `static/tafsirs.json`");
        std::process::exit(1)
    }
    let tafsir_index = tafsir_index.unwrap(); // safe to use unwrap()

    return Ok(list[tafsir_index].clone());
}
