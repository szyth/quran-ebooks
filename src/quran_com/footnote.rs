use regex::Regex;

use crate::env;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
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
pub(crate) async fn get_footnote(text: &String) -> Result<Vec<String>, Error> {
    let mut footnote: Vec<String> = vec![];
    let re = Regex::new(r"<sup foot_note=(?<footnote>\d*)>").unwrap();
    for caps in re.captures_iter(&text) {
        let url = format!(
            "{}/foot_notes/{}",
            env::api_url().unwrap(),
            &caps["footnote"]
        ); // safe to use unwrap

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Referer", "https://quran.com".parse().unwrap()); // safe to use unwrap

        let res: Response = reqwest::Client::new()
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json()
            .await?;
        footnote.push(res.foot_note.text);
    }

    Ok(footnote)
}
