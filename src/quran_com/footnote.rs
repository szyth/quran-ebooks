#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("ReqWestError: {0}")]
    ReqWestError(#[from] reqwest::Error),
}

#[derive(serde::Deserialize, Debug)]
struct Response {
    foot_note: Footnote,
}

#[derive(serde::Deserialize, Debug)]
struct Footnote {
    id: u32,
    language_id: u32,
    text: String,
    language_name: String,
}
async fn get_footnote(footnote_number: u32) -> Result<String, Error> {
    let quran_api = "https://quran.com/api/proxy/content/api/qdc/foot_notes";
    let footnote_url = format!("{quran_api}/{}", footnote_number);
    println!("{footnote_url}");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Referer", "https://quran.com/1".parse().unwrap()); // safe to use unwrap

    let res: Response = reqwest::Client::new()
        .get(footnote_url)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    let footnote = format!("Footnote");
    Ok(res.foot_note.text)
}
