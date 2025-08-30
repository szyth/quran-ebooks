### 📘 Download eBooks

You can download the available eBooks from this folder:

👉 [Click here to open the Google Drive `quran-ebooks` folder](https://drive.google.com/drive/folders/1RQ7F3-RSJ_uS9wJMIiVAa1d_gigpGpFk?usp=sharing)

Then click on any file you want, and press the **Download** button.

Please report any issues or bugs at the email provided in the Google Drive link above.


---
### Or you can generate your custom ebooks by the following documentation: (only for Technical audience)

### Qurán for eReaders
Features:
- Quran Arabic. Indopak font
- Word by Word translation
- English translations supported.
- Page number and Sajdah.
- Tafsir support

Snippet from Kobo eReader:
![Reader_67  Al-Mulk epub_p2_2025-07-01_125848](https://github.com/user-attachments/assets/1c6f5fc0-dedd-4a16-869d-a233f3029dd3)



## Create eBOOKs

This is a 3 step process. 
- Login to `quran.com` API
- Create HTMLs of Translations or Tafsir. This fetches all Surah content as JSON from `quran.com`. Includes core logic of this tool.
- Convert these HTMLs into EPUB or PDF.
  - PDF is recommned for most users, as EPUB will require an `KoReader` installation in eReaders.

### 1. Login
- Create `.env` using your API creds
    - refer `sampleenv` for format
    - enter your client ID and client Secret, leave the access token empty.
    - You can request API access from here https://api-docs.quran.foundation/request-access

```bash
cargo run -- --login
```
- This will give you the Access Token, now store it in `.env`


### 2. Generate HTMLs for Translation or Tafsir
```bash
cargo run -- --translations --start-surah 1 --end-surah 114
cargo run -- --tafsir       --start-surah 1 --end-surah 114

# Created HTMLs can be found in output folder
```

### 3a. Convert HTMLs to PDFs
```bash
# Install Calibre software, it comes with an `ebook-convert` plugin.
# run the following shell command from root of project.
mkdir -p pdfs && for f in output/*.html; do ebook-convert "$f" "pdfs/$(basename "${f%.html}.pdf")" --disable-font-rescaling --pdf-default-font-size 32  --pdf-page-margin-left 15 --pdf-page-margin-right 15 --pdf-page-margin-top 15 --pdf-page-margin-bottom 15; done

# or do manually with the following (make sure `fonts` folder should exist in the same directory as of `filename.html`)
ebook-convert filename.html filename.pdf   --disable-font-rescaling --pdf-default-font-size 32  --pdf-page-margin-left 15 --pdf-page-margin-right 15 --pdf-page-margin-top 15 --pdf-page-margin-bottom 15

# Created EPUBs can be found in `pdfs` folder

```


### 3b. Convert HTMLs to eBOOK (EPUB format)
```bash
# Install Calibre software, it comes with an `ebook-convert` plugin.
# run the following shell command from root of project.
mkdir -p ebooks && for f in output/*.html; do ebook-convert "$f" "ebooks/$(basename "${f%.html}.epub")" --disable-font-rescaling; done

# or do manually with the following (make sure `fonts` folder should exist in the same directory as of `filename.html`)
ebook-convert filename.html filename.epub --disable-font-rescaling

# Created EPUBs can be found in `ebooks` folder
```
#### Kindle/Kobo Setup after downloading eBook:
- Use [KOReader](https://koreader.rocks/) app to render arabic properly.
  - in KOReader: Set `Render Mode: Book` and `Enable: Embedded Style, Embedded Fonts`








Credits:
- All Data sourced from: https://www.quran.com
