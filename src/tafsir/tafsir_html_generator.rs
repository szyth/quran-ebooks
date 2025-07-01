use crate::{
    quran_com::{get_surah_details, get_tafsir_details},
    tafsir::tafsir::Tafsir,
};

#[tracing::instrument(skip_all)]
pub(crate) async fn handler(
    start_surah: u8,
    end_surah: u8,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let resource_id = 169; // hardcoded tafsir ID. refer folder static/tafsirs.json for tafsir ID.
    for surah_number in start_surah..=end_surah {
        tracing::info!(
            "Initiate Tafsir HTML generation for Surah number: {}",
            surah_number
        );
        let surah_details = get_surah_details::handler(surah_number)?;
        let tafsir_details = get_tafsir_details::handler(resource_id)?;
        tracing::info!("Fetched {:#?}", surah_details);
        let data = Tafsir::by_surah(surah_number, resource_id).await?;

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
        if let Err(e) = std::fs::write(filename, output_html) {
            eprintln!("Error: Failed to write to file: {:#?}", e);
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
