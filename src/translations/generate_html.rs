use crate::quran_com::types::{
    surah_details,
    verse::{self, Verse},
};

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("ReqWestError: {0}")]
    ReqWestError(#[from] reqwest::Error),
    #[error("RegexError: {0}")]
    RegexError(#[from] regex::Error),

    #[error("SurahDetails: {0}")]
    SurahDetails(#[from] surah_details::Error),

    #[error("VerseError: {0}")]
    VerseError(#[from] verse::Error),

    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
}

#[tracing::instrument(skip_all)]
pub(crate) async fn handler(start_surah: u8, end_surah: u8) -> Result<(), Error> {
    let mut output_html = String::new();
    output_html.push_str(&get_css_header());

    for surah_number in start_surah..=end_surah {
        tracing::info!(
            "Initiate HTML generation for Surah number: {}",
            surah_number
        );

        output_html.push_str(&get_cover_page(surah_number)?);
        output_html.push_str(&get_all_verses(surah_number).await?);
    }

    // save a response to file
    let filename = format!("output/quran-surah-{}-{}.html", start_surah, end_surah);
    tracing::info!("Storing HTML data here: {filename}",);
    let _write = std::fs::write(filename, output_html)?;
    Ok(())
}
#[tracing::instrument(skip_all)]
async fn get_all_verses(surah_number: u8) -> Result<String, Error> {
    tracing::info!("Adding verses");

    let mut verses_html = String::new();
    let data = Verse::by_surah(surah_number).await?;
    for verse in data.verses {
        verses_html.push_str(&format!(
            "<div class=\"container\">
                    {}
                    {}
                    {}
                    {}
                </div>
                ",
            verse.get_header(), // page number, sajdah, ayah
            verse.get_arabic_indopak(),
            verse.get_word_by_word(),
            verse.get_translations().await?
        ));
    }

    Ok(verses_html)
}
#[tracing::instrument(skip_all)]
fn get_cover_page(surah_number: u8) -> Result<String, Error> {
    let surah_details = surah_details::handler(surah_number)?;
    tracing::info!("Fetched {:#?}", surah_details);

    // surah cover page
    tracing::info!("Creating cover page");

    let mut tauz_tasmiya = format!(
        "
                <div class=\"arabic\">أَعُوذُ بِٱللَّهِ مِنَ ٱلشَّيۡطَـٰنِ ٱلرَّجِيمِ</div>
                <div class=\"arabic\">بِسۡمِ اللهِ الرَّحۡمٰنِ الرَّحِيۡمِ</div>
            "
    );
    // dont include tauz-tasmiya in Surah Tawbah ie Surah 9
    if surah_number == 9 {
        tauz_tasmiya = "".into();
    }
    let cover_page = format!(
        "
            <div class=\"cover\">
                {tauz_tasmiya}
                <div>Surah {0} | {1}</div>
                <div>Verses: {2}</div>
                <div>Revelation: {3}</div>
            </div>
        ",
        surah_details.transliterated_name,
        surah_details.translated_name,
        surah_details.verses_count,
        surah_details.revelation_place
    );

    Ok(cover_page)
}
#[tracing::instrument(skip_all)]
fn get_css_header() -> String {
    format!(
        "
        <style>
            @font-face {{
                font-family: 'IndoPak';
                src:
                    local('AlQuran IndoPak by QuranWBW'),
                    url('fonts/indopak-nastaleeq-waqf-lazim-v4.2.1.ttf') format('truetype');
                font-display: swap;
            }}

            @font-face {{
                font-family: 'UthmanicHafs';
                src:
                    local('KFGQPC HAFS Uthmanic Script'),
                    url('fonts/quran/hafs/uthmanic_hafs/UthmanicHafs1Ver18.ttf') format('truetype');
                font-display: swap;
            }}
            .cover {{
                text-align: center;
                border: 5px solid black;
                padding: 0.7em;

            }}
            .container {{
                border-bottom: 1px solid black;
                margin-bottom: 30px;
                padding-bottom: 15px;
            }}

            .arabic {{
                font-size: 2em;
                line-height: 1.5;
                font-family: 'IndoPak', sans-serif;
                direction: rtl;
            }}
            .wbw {{
                font-size: 0.8em;
                margin-top: 5px;
                margin-bottom: 2px;
            }}

            .translation {{
                font-size: 1em;
            }}
            .header {{}}
            .page {{
                font-size: 0.6em;
            }}
            .sajdah {{
                font-weight: 600;
            }}
            .ayah {{}}
            .footnote {{
                font-size: 0.7em;
                border: 1px solid #424242;
                padding: 0.4em;
            }}
            .footnote p {{
                margin: 3px;
            }}
        </style>
        ",
    )
}
