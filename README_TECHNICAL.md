# Tool Documentation - Only for Technical Audience
Generate your custom ebooks by the following documentation


Snippet from Kobo eReader:
![Reader_67  Al-Mulk epub_p2_2025-07-01_125848](https://github.com/user-attachments/assets/1c6f5fc0-dedd-4a16-869d-a233f3029dd3)

---
---
## FIXED: IndoPak ayah number shown one word before the ayah end
- For IndoPak script, the ayah number used to appear one word before the ayah end at: 53:21,27,37,45, 75:39, 80:3, 92:3 (and 4:157, 37:45, where the last waqf mark was pushed past the ayah number).
<img width="576" height="204" alt="image" src="https://github.com/user-attachments/assets/8c2949ac-b61f-4b81-9e90-e1bc9114b5fa" />

### Cause
- The quran.com IndoPak text encodes some whole words, waqf signs and the ayah-number circle as Private Use Area (PUA) characters, which Unicode treats as left-to-right. When a PUA word is followed by a space and then the PUA ayah number, the renderer joins them into one left-to-right run inside the right-to-left line and swaps their order.

### Fix
- The generator now inserts an invisible RIGHT-TO-LEFT MARK (U+200F) after any PUA character followed by a space (`fix_indopak_bidi` in `src/quran_com/types/verse.rs`). No manual `<br>` editing is needed any more. PUA characters directly touching the ayah number (waqf signs that sit on the circle) are left untouched.


## Create eBOOKs

This is a 4 step process:
1. Setup a `.env` file
2. Configure `translation_config.json` and/or `tafsir_config.json` (both exist by default)
3. Generate HTML files using CLI flags (`--translations` or `--tafsir`)
4. Convert HTMLs to PDF or EPUB
    - PDF is recommended for most users, as EPUB requires additional setup of `KoReader` app. Watch tutorial: https://youtu.be/Jret-648FZ4


### 1. Setup .env file
- Get API Access from https://api-docs.quran.foundation/request-access
- Create `.env` using above API credentials, refer `sampleenv` file for format

### 2. Configure Settings
Both `translation_config.json` and `tafsir_config.json` exist by default in root directory. Edit them as needed. See [CONFIG_GUIDE.md](CONFIG_GUIDE.md) for detailed options.

**Translation Config Example:**
```json
{
  "start_surah": 1,
  "end_surah": 114,
  "arabic": { "script": "indopak" },
  "word_by_word": true,
  "transliteration": true,
  "translation": { "id": 20 }
}
```

**Tafsir Config Example:**
```json
{
  "start_surah": 1,
  "end_surah": 114,
  "resource_id": 168
}
```

### 3. Generate HTML Files
```bash
# Generate translations:
cargo run -- --translations

# Generate tafsir:
cargo run -- --tafsir

# Output HTML files will be in `output/` folder
```

### 4a. Convert HTMLs to PDFs
```bash
# Install Calibre software, it comes with an `ebook-convert` plugin.
# run the following shell command from root of project.
mkdir -p pdfs && for f in output/*.html; do ebook-convert "$f" "pdfs/$(basename "${f%.html}.pdf")" --disable-font-rescaling --pdf-default-font-size 32  --pdf-page-margin-left 15 --pdf-page-margin-right 15 --pdf-page-margin-top 15 --pdf-page-margin-bottom 15; done

# Created PDFs can be found in `pdfs` folder

# OR, do manually with the following command. (make sure `fonts` folder should exist in the same directory as of `filename.html`)
ebook-convert filename.html filename.pdf --disable-font-rescaling --pdf-default-font-size 32  --pdf-page-margin-left 15 --pdf-page-margin-right 15 --pdf-page-margin-top 15 --pdf-page-margin-bottom 15

# OR, you can manually drag-and-drop the HTML into Calibre software and click on Convert. Make sure to 'Disable Font Rescaling, PDF Default Font Size: 32, PDF Margin Top Bottom Right Left: 15'
```


### 4b. Convert HTMLs to eBOOK (EPUB format)
```bash
# Install Calibre software, it comes with an `ebook-convert` plugin.
# run the following shell command from root of project.
mkdir -p epubs && for f in output/*.html; do ebook-convert "$f" "epubs/$(basename "${f%.html}.epub")" --disable-font-rescaling; done

# Created EPUBs can be found in `epubs` folder

# OR, do manually with the following command. (make sure `fonts` folder should exist in the same directory as of `filename.html`)
ebook-convert filename.html filename.epub --disable-font-rescaling

# OR, you can manually drag-and-drop the HTML into Calibre software and click on Convert. Make sure to 'Disable Font Rescaling'
```
#### Kindle/Kobo Setup after downloading EPUB:
- Use [KOReader](https://koreader.rocks/) app to render arabic properly.
  - in KOReader: Set `Render Mode: Book` and `Enable: Embedded Style, Embedded Fonts`


---

Credits:
- All Data sourced from: https://www.quran.com
