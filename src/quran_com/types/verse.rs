use crate::quran_com::apis::{get_footnote, get_verses_by_chapter};

#[derive(serde::Deserialize, Debug)]
pub(crate) struct VerseData {
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
    pub(crate) text_indopak_nastaleeq: String,
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
    pub(crate) text_uthmani: String,
    // pub(crate) text_indopak_nastaleeq: String,
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

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("ReqWestError: {0}")]
    ReqWestError(#[from] reqwest::Error),
    #[error("RegexError: {0}")]
    RegexError(#[from] regex::Error),

    #[error("GetVerseByChapterError: {0}")]
    GetVerseByChapterError(#[from] get_verses_by_chapter::Error),

    #[error("FootnoteError: {0}")]
    FootnoteError(#[from] get_footnote::Error),
}

impl Verse {
    pub(crate) async fn by_surah(surah_number: u8) -> Result<VerseData, Error> {
        tracing::info!("Fetching Verses from quran.com server");
        let data = crate::quran_com::apis::get_verses_by_chapter::handler(surah_number).await?;
        Ok(data)
    }

    pub(crate) fn get_arabic_indopak(&self) -> String {
        tracing::debug!("Inside get_arabic_indopak()");
        format!(
            "<div class=\"arabic\">{}</div>",
            self.text_indopak_nastaleeq
        )
    }
    pub(crate) fn get_arabic_uthmani(self) -> String {
        format!("<div class=\"arabic\">{}</div>", self.text_uthmani)
    }
    pub(crate) fn get_word_by_word(&self) -> String {
        tracing::debug!("Inside get_word_by_word()");
        let mut wbw_html = String::new();
        for word in &self.words {
            // Arabic with English RTL, works on Calibre, but not on eReaders due to modern CSS.
            //
            // wbw_html.push_str(&format!(
            //     "
            //     <div class=\"word_wrapper\">
            //         <span class=\"word_arabic\">{}</span>
            //         <span class=\"word_eng\">{}</span>
            //     </div>
            //     ",
            //     word.text_indopak,
            //     word.translation
            //         .text
            //         .clone()
            //         .expect("failed to unwrap word.translation.text")
            // ));
            //
            //

            // check this
            // wbw_html.push_str(&format!(
            //     "{}. {}    ",
            //     word.translation
            //         .text
            //         .clone()
            //         .expect("failed to unwrap word.translation.text"),
            //     word.text_indopak
            // ));

            wbw_html.push_str(&format!(
                "{}. ",
                word.translation
                    .text
                    .clone()
                    .expect("failed to unwrap word.translation.text"),
            ));
        }
        format!("<div class=\"wbw\">{}</div>", wbw_html)
    }
    pub(crate) fn get_verse_number(&self) -> u32 {
        self.verse_number
    }
    fn get_sajdah(&self) -> String {
        if self.sajdah_number.is_some() {
            return "sajdah".to_string();
        }

        "".to_string()
    }
    pub(crate) fn get_header(&self) -> String {
        tracing::debug!("Inside get_header()");
        format!(
            "<table width=\"100%\">
                    <tr>
                        <td align=\"left\" style=\"white-space: nowrap;\">
                            <span class=\"page\">Pg.{0}</span>
                            <span class=\"sajdah\">{1}</span>
                        </td>
                        <td align=\"right\" style=\"white-space: nowrap;\">
                            <span class=\"ayah\">{2}</span>
                        </td>
                    </tr>
                </table>",
            self.page_number,
            self.get_sajdah(),
            self.verse_number
        )
    }
    pub(crate) async fn get_translations(self) -> Result<String, Error> {
        tracing::debug!("Inside get_translations()");
        let mut translation_html: String = String::new();
        for translation in self.translations.iter() {
            translation_html.push_str(&format!(
                " <div class=\"translation\">{0}. {1}</div>
                    {2}
                ",
                self.get_verse_number(),
                translation.text,
                get_translation_footnote(&translation.text,).await?
            ));
        }
        Ok(translation_html)
    }
}
async fn get_translation_footnote(translation: &str) -> Result<String, Error> {
    tracing::debug!("Inside get_translation_footnote()");
    let footnotes = get_footnote::handler(translation).await?;

    let mut footnote_html: String = String::new();
    for (index, footnote) in footnotes.iter().enumerate() {
        footnote_html.push_str(&format!("Footnote {}: {}<br>", index + 1, footnote));
    }
    if !footnote_html.is_empty() {
        footnote_html = format!("<div class=\"footnote\">{}</div>", footnote_html);
    }
    Ok(footnote_html)
}
