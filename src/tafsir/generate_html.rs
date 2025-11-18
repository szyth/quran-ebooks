use crate::tafsir::config::TafsirConfig;
use crate::quran_com::types::{
    surah_details,
    tafsir::{self, Tafsir},
    tafsir_details,
};

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("ReqWestError: {0}")]
    ReqWestError(#[from] reqwest::Error),
    #[error("RegexError: {0}")]
    RegexError(#[from] regex::Error),

    #[error("SurahDetails: {0}")]
    SurahDetails(#[from] surah_details::Error),

    #[error("TafsirDetailsError: {0}")]
    TafsirDetailsError(#[from] tafsir_details::Error),

    #[error("TafsirError: {0}")]
    TafsirError(#[from] tafsir::Error),

    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
}

#[tracing::instrument(skip_all)]
pub(crate) async fn handler(config: TafsirConfig) -> Result<(), Error> {
    for surah_number in config.start_surah..=config.end_surah {
        tracing::info!(
            "Initiate Tafsir HTML generation for Surah number: {}",
            surah_number
        );
        let surah_details = surah_details::handler(surah_number)?;
        let tafsir_details = tafsir_details::handler(config.resource_id as usize)?;
        tracing::info!("Fetched {:#?}", surah_details);
        let data = Tafsir::by_surah(surah_number, config.resource_id as usize).await?;

        let mut output_html = get_html_styling();

        // surah cover page
        tracing::info!("Creating cover page");
        output_html.push_str(&format!(
            "
            <div class=\"cover\">
                <div class=\"arabic\">أَعُوذُ بِٱللَّهِ مِنَ ٱلشَّيۡطَـٰنِ ٱلرَّجِيمِ</div>
                <div class=\"arabic\">بِسۡمِ اللهِ الرَّحۡمٰنِ الرَّحِيۡمِ</div>
                <div>Surah {0} | {1}</div>
                <div>Verses: {2}</div>
                <div>Revelation: {3}</div>
            </div>
        ",
            surah_details.transliterated_name,
            surah_details.translated_name,
            surah_details.verses_count,
            surah_details.revelation_place
        ));

        tracing::info!("Adding verses");
        for verse in data.tafsirs {
            output_html.push_str(&format!("{0}", verse.text));
        }

        let filename = format!(
            "output/{}. {}-{}.html",
            surah_number, surah_details.transliterated_name, tafsir_details.translated_name,
        );
        tracing::info!("Storing HTML data here: {filename}",);
        // save a response
        let _write = std::fs::write(filename, output_html)?;
    }

    Ok(())
}
#[tracing::instrument(skip_all)]
fn get_html_styling() -> String {
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
            * {{
                font-family: 'IndoPak', sans-serif;
            }}
            .cover {{
                text-align: center;
                border: 5px solid black;
                padding: 0.7em;

            }}
           
            .arabic {{
                font-size: 2em;
                line-height: 1.5;
                font-family: 'IndoPak', sans-serif;
                direction: rtl;
            }}
                             
        </style>
        ",
    )
}
