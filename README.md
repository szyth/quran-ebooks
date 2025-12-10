# 📘 Download eBooks

You can download available eBooks from these links:

- ### Quran - Arabic Only - IndoPak script [Download Link](https://drive.google.com/drive/folders/1e9ID9a5LEiMKZbvK84_VSYZ0LctS5qCm?usp=sharing)
- ### Quran - Arabic with Translation Saheeh International - IndoPak script [Download Link](https://drive.google.com/drive/folders/1GALrk2ZB0HxoZJh8g3EX2enE4VTzBeTd?usp=sharing)
- ### Quran - Tafsir [Download Link](https://drive.google.com/drive/folders/1eVon53mh1hXJ5IuuNE-4mEx4DC6UX9ED?usp=sharing)


All these ebooks and others not listed here are available under one Google Drive folder [quran-ebooks](https://drive.google.com/drive/folders/1RQ7F3-RSJ_uS9wJMIiVAa1d_gigpGpFk?usp=sharing)

# ❓ Can’t find the eBook you’re looking for? Fill out [this Google form](https://forms.gle/j4wQ8YZpaBrJZtEX8) and I will create it for you, insha’Allah.

# Important Note:
- All content in these eBooks is sourced exclusively from `quran.com`. Full credit goes to the developers of `quran.com` for their dedicated work; I claim none of it as my own.
- The download link above gives 2 options to download: EPUB or PDF. PDF being the EASIEST to view and recommended for most people, while EPUB requires additional setup of KOReader. Watch my Youtube video for EPUB tutorial: https://youtu.be/Jret-648FZ4

---
Please report any issues or bugs at the email provided in the Google Drive link above.

---
# Tool Documentation - Only for Technical Audience
Generate your custom ebooks by the following documentation


Snippet from Kobo eReader:
![Reader_67  Al-Mulk epub_p2_2025-07-01_125848](https://github.com/user-attachments/assets/1c6f5fc0-dedd-4a16-869d-a233f3029dd3)

---
---
## KNOWN ISSUE in quran.com developer API (4.0 version at the time of writing)
- For IndoPak script, we get ayah number one word before the ayah end. This has been observed at: 53:21,27,37,45, 75:39, 80:3, 92:3
<img width="576" height="204" alt="image" src="https://github.com/user-attachments/assets/8c2949ac-b61f-4b81-9e90-e1bc9114b5fa" />

### Workaround
- Until quran.com developer API is updated, we have to manually add a line break (`<br>` tag) between ayah end and Ayah number in generated HTML at the known ayah mentioned above.


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
