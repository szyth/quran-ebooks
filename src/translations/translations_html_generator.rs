use crate::{quran_com::get_surah_details, translations::verse::Verse};

#[tracing::instrument(skip_all)]
pub(crate) async fn handler(
    start_surah: u8,
    end_surah: u8,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    for surah_number in start_surah..=end_surah {
        tracing::info!(
            "Initiate HTML generation for Surah number: {}",
            surah_number
        );
        let surah_details = get_surah_details::handler(surah_number)?;
        tracing::info!("Fetched {:#?}", surah_details);
        let data = Verse::by_surah(surah_number).await?;

        let mut output_html = get_html_styling();

        // surah cover page
        tracing::info!("Creating cover page");

        let mut tauz_tasmiya = format!(
            "
                <div class=\"arabic\">أَعُوذُ بِٱللَّهِ مِنَ ٱلشَّيۡطَـٰنِ ٱلرَّجِيمِ</div>
                <div class=\"arabic\">بِسۡمِ اللهِ الرَّحۡمٰنِ الرَّحِيۡمِ</div>
            "
        );
        // dont include tauz-tasmiya in Surah Tawbah ie surah 9
        if surah_number == 9 {
            tauz_tasmiya = "".into();
        }
        output_html.push_str(&format!(
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
        ));

        tracing::info!("Adding verses");
        for verse in data.verses {
            output_html.push_str(&format!(
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

        let filename = format!(
            "output/{}. {}.html",
            surah_number, surah_details.transliterated_name
        );
        tracing::info!("Storing HTML data here: {filename}",);
        // save a response
        if let Err(e) = std::fs::write(filename, output_html) {
            tracing::error!("Error: Failed to write to file: {:#?}", e);
            std::process::exit(1)
        }
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
