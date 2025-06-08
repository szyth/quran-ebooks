use crate::{quran_com::get_surah_details, verse::Verse};

#[derive(serde::Serialize, Debug)]
struct OutputVerse {
    verse: u32,
    arabic_text: String,
    word_by_word: String,
    translation: String,
    page_number: u32,
    sajdah: String,
    footnotes: String,
}
pub(crate) async fn handler(
    start_surah: u8,
    end_surah: u8,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    for surah_number in start_surah..=end_surah {
        let surah_details = get_surah_details::handler(surah_number)?;
        let surah_name = surah_details.transliterated_name;
        let data = Verse::by_surah(surah_number).await?;

        let mut output_html = get_html_styling();
        for verse in data.verses {
            output_html.push_str(&format!(
                "<div class=\"container\">
                    {0}
                    {1}
                    {2}
                    {3}
                </div>
                ",
                verse.get_header(), // page number, sajdah, ayah
                verse.get_arabic_indopak(),
                verse.get_word_by_word(),
                verse.get_translations().await?
            ));
        }

        // save a response
        let _res = std::fs::write(
            format!("output/{}. {}.html", surah_number, surah_name),
            output_html,
        )?;
    }

    Ok(())
}
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

            .container {{
                border-bottom: 1px solid black;
                margin-bottom: 30px;
                padding-bottom: 15px;
            }}

            .arabic {{
                font-size: 2em;
                line-height: 1.5;
                font-family: 'IndoPak', sans-serif;
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
