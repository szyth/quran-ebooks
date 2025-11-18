use crate::translations::config::TranslationConfig;
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
pub(crate) async fn handler(config: TranslationConfig) -> Result<(), Error> {
    let mut output_html = String::new();
    output_html.push_str(&get_css_header(&config));

    // Add table of contents
    tracing::info!("Creating table of contents");
    output_html.push_str(&get_table_of_contents(config.start_surah, config.end_surah)?);

    for surah_number in config.start_surah..=config.end_surah {
        tracing::info!(
            "Initiate HTML generation for Surah number: {}",
            surah_number
        );

        output_html.push_str(&get_cover_page(surah_number)?);
        output_html.push_str(&get_all_verses(surah_number, &config).await?);
    }

    // save a response to file
    let filename = format!(
        "output/quran-surah-{}-{}.html",
        config.start_surah, config.end_surah
    );
    tracing::info!("Storing HTML data here: {filename}",);
    let _write = std::fs::write(filename, output_html)?;
    Ok(())
}
#[tracing::instrument(skip_all)]
async fn get_all_verses(surah_number: u8, config: &TranslationConfig) -> Result<String, Error> {
    tracing::info!("Adding verses");

    let mut verses_html = String::new();

    // Fetch verses with optional translation_id
    let translation_id = config.translation.as_ref().map(|t| t.id);
    let data = Verse::by_surah(surah_number, translation_id).await?;

    for verse in data.verses {
        let mut verse_content = String::new();

        // Always include header (page number, sajdah, verse number)
        verse_content.push_str(&verse.get_header());

        // Include Arabic text if requested
        if let Some(arabic_config) = &config.arabic {
            let arabic_html = match arabic_config.script {
                crate::translations::config::ArabicScript::IndoPak => verse.get_arabic_indopak(),
                crate::translations::config::ArabicScript::Uthmani => verse.get_arabic_uthmani(),
            };
            verse_content.push_str(&arabic_html);
        }

        // Include word-by-word if requested
        if config.word_by_word {
            verse_content.push_str(&verse.get_word_by_word());
        }

        // Include translation if requested
        if config.translation.is_some() {
            verse_content.push_str(&verse.get_translations().await?);
        }

        verses_html.push_str(&format!(
            "<div class=\"container\">
                    {}
                </div>
                ",
            verse_content
        ));
    }

    Ok(verses_html)
}

#[tracing::instrument(skip_all)]
fn get_table_of_contents(start_surah: u8, end_surah: u8) -> Result<String, Error> {
    let mut toc_html = String::from(
        "<div class=\"toc\">
            <h1>Table of Contents</h1>
            <ul>",
    );

    for surah_number in start_surah..=end_surah {
        let surah_details = surah_details::handler(surah_number)?;
        toc_html.push_str(&format!(
            "<li><a href=\"#surah-{}\">{0}. {1} - {2} ({3} verses)</a></li>",
            surah_number,
            surah_details.transliterated_name,
            surah_details.translated_name,
            surah_details.verses_count
        ));
    }

    toc_html.push_str("</ul></div>");
    Ok(toc_html)
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
            <div id=\"surah-{4}\" class=\"cover\">
                {tauz_tasmiya}
                <div>Surah {0} | {1}</div>
                <div>Verses: {2}</div>
                <div>Revelation: {3}</div>
            </div>
        ",
        surah_details.transliterated_name,
        surah_details.translated_name,
        surah_details.verses_count,
        surah_details.revelation_place,
        surah_number
    );

    Ok(cover_page)
}
#[tracing::instrument(skip_all)]
fn get_css_header(config: &TranslationConfig) -> String {
    // Determine which font to use for Arabic text
    let arabic_font = if let Some(arabic_config) = &config.arabic {
        match arabic_config.script {
            crate::translations::config::ArabicScript::IndoPak => "'IndoPak', sans-serif",
            crate::translations::config::ArabicScript::Uthmani => "'UthmanicHafs', sans-serif",
        }
    } else {
        "'IndoPak', sans-serif" // Default fallback
    };

    format!(
        "
        <style>
            @font-face {{
                font-family: 'IndoPak';
                src:
                    local('AlQuran IndoPak by QuranWBW'),
                    url('fonts/AlQuranNeov5x1_hanafi.ttf') format('truetype');
                font-display: swap;
            }}

            @font-face {{
                font-family: 'UthmanicHafs';
                src:
                    local('KFGQPC HAFS Uthmanic Script'),
                    url('fonts/UthmanicHafs_V20.ttf') format('truetype');
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
                font-family: {};
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
            .toc {{
                border: 3px solid black;
                padding: 1em;
                margin-bottom: 30px;
                background-color: #f9f9f9;
            }}
            .toc h1 {{
                text-align: center;
                font-size: 1.5em;
                margin-bottom: 0.5em;
            }}
            .toc ul {{
                list-style-type: none;
                padding: 0;
            }}
            .toc li {{
                margin: 0.5em 0;
                font-size: 1em;
            }}
            .toc a {{
                text-decoration: none;
                color: #333;
            }}
            .toc a:hover {{
                text-decoration: underline;
                color: #0066cc;
            }}
        </style>
        ",
        arabic_font
    )
}
