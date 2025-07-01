### 📘 Download eBooks

You can download the available eBooks from this folder:

👉 [Click here to open the `download-ebooks` folder](https://github.com/szyth/quran-ebook-generator/tree/main/download-ebooks)

Then click on any file you want, and press the **Download** button (top right) on the file's page.

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
- Create Translations or Tafsir HTMLs using EPUB-compatible `css` and `font`. This fetches all Surah content as JSON from `quran.com`. Includes core logic of this tool.
- Convert these HTMLs into EPUB

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

### 3. Convert HTMLs to eBOOK (EPUB format)
```bash
# Install Calibre software, it comes with an `ebook-convert` plugin.
# run the following shell command from root of project.
mkdir -p ebooks && for f in output/*.html; do ebook-convert "$f" "ebooks/$(basename "${f%.html}.epub")" --disable-font-rescaling; done

# or do manually with:
ebook-convert filename.html filename.epub --disable-font-rescaling

# Created EPUBs can be found in `ebooks` folder
```


#### eReader Setup
- Use KOReader app to render arabic properly.
  - in KOReader:
    - Enable Embedded Style, Embedded Fonts, and Render Mode: Book


Credits:
- All Data sourced from: https://www.quran.com
