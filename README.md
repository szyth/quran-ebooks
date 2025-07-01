# Qurán for eReaders
Features:
- Quran Arabic. Indopak font
- Word by Word translation
- English translations supported.
- Page number and Sajdah.
- Tafsir support


# Create eBOOKs

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
