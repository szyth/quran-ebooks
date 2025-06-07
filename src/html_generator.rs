use regex::Regex;

use crate::quran_com::{footnote::get_footnote, get_surah_details, get_verses_by_chapter};

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
        let surah_name = surah_details.transliteratedName;
        let data = get_verses_by_chapter::handler(surah_number).await?;

        let mut output_json: Vec<OutputVerse> = vec![];
        let mut output_html = format!(
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
        );
        for verse in data.verses {
            // println!("{:?}", verse.text_indopak);
            // println!("{:?}", verse.verse_number);

            let mut output_word = String::new();
            for word in verse.words {
                // output_word.push_str(&format!(
                //     "{} \n {}.",
                //     word.text,
                //     word.translation
                //         .text
                //         .expect("failed to unwrap word.translation.text")
                // ));

                output_word.push_str(&format!(
                    "{}. ",
                    word.translation
                        .text
                        .expect("failed to unwrap word.translation.text")
                ));
            }
            let translations = verse.translations[0].text.clone();
            let footnotes = get_footnote(&translations).await?;

            let output_verse = OutputVerse {
                arabic_text: verse.text_indopak,
                verse: verse.verse_number,
                word_by_word: output_word,
                translation: translations,
                page_number: verse.page_number,
                sajdah: {
                    if verse.sajdah_number.is_some() {
                        "sajdah".to_string()
                    } else {
                        "".to_string()
                    }
                },
                footnotes,
            };
            output_html.push_str(&format!(
                "
            <div class=\"container\">
                <table width=\"100%\">
                    <tr>
                        <td align=\"left\" style=\"white-space: nowrap;\">
                            <span class=\"page\">Pg.{4}</span>
                            <span class=\"sajdah\">{5}</span>
                        </td>
                        <td align=\"right\" style=\"white-space: nowrap;\">
                            <span class=\"ayah\">{2}</span>
                        </td>
                    </tr>
                </table>
                <div class=\"arabic\">{0}</div>
                <div class=\"wbw\">{1}</div>
                <div class=\"translation\">{2}. {3}</div>
                {6}
            </div>",
                output_verse.arabic_text,
                output_verse.word_by_word,
                output_verse.verse,
                output_verse.translation,
                output_verse.page_number,
                output_verse.sajdah,
                output_verse.footnotes
            ));
            output_json.push(output_verse);
        }
        // println!("{:?}", output);

        // save a response
        // let _res = std::fs::write("output/output.json", serde_json::to_string_pretty(&output_json)?)?;
        let _res = std::fs::write(
            format!("output/{}. {}.html", surah_number, surah_name),
            output_html,
        )?;
    }

    Ok(())
}
