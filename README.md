# Qurán for eReaders
Features:
- Quran Arabic. Indopak font
- Word by Word translation
- English translation by Dr. Mustafa Khattab
- Page number and Sajdah.

# Known bugs:
- alif-noon comes to top


# Create EPUBs
- Login to `quran.com` API
- Create HTMLs using EPUB-compatible `css` and `font`. This fetches all Surah content as JSON from `quran.com`. Includes core logic of this tool.
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


### 2. Get Arabic text and Translations 
```bash
# For One Surah
cargo run -- --translations --start-surah <Surah Number>
# example: 
cargo run -- --translations --start-surah 67

# For Multiple Surahs
cargo run -- --translations --start-surah <Surah Number> --end-surah <Surah Number>
# example: 
cargo run -- --translations --start-surah 1 --end-surah 114

# Created HTMLs can be found in output folder
```

### 3. Get Tafsir
```bash
# For One Surah
cargo run -- --tafsir --start-surah <Surah Number>
# example: 
cargo run -- --tafsir --start-surah 67

# For Multiple Surahs
cargo run -- --tafsir --start-surah <Surah Number> --end-surah <Surah Number>
# example: 
cargo run -- --tafsir --start-surah 1 --end-surah 114

# Created HTMLs can be found in output folder
```

### 4. Convert HTMLs to EPUB
```bash
# Install Calibre software, it comes with an `ebook-convert` plugin.
# run the following shell command from root of project.
mkdir -p ebooks && for f in output/*.html; do ebook-convert "$f" "ebooks/$(basename "${f%.html}.epub")" --disable-font-rescaling; done

# or do manually with:
ebook-convert filename.html filename.epub --disable-font-rescaling

# Created EPUBs can be found in `ebooks` folder
```


#### Inside Kobo use KOReader.
Open Book and from bottom right-most settings, enable Embedded Style and Embedded Fonts.




Credits:
- All Data sourced from: https://www.quran.com