Todo
- Add footnotes
- Create front page with surah details
- add author for epub
- add support for multi translations
- add support for tafsir
- add tracing logs


# Qurán for eReaders
Features:
- Quran Arabic. Indopak font
- Word by Word translation
- English translation by Dr. Mustafa Khattab
- Page number and Sajdah.




# Create EPUBs

This is a 3 step process. This script does it all for you.
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


### 2. Generate HTMLs
```bash
# For One Surah
cargo run -- --generate-html --start-surah <Surah Number>
# example: 
cargo run -- --generate-html --start-surah 67

# For Multiple Surahs
cargo run -- --generate-html --start-surah <Surah Number> --end-surah <Surah Number>
# example: 
cargo run -- --generate-html --start-surah 1 --end-surah 114

# Created HTMLs can be found in output folder
```

### 3. Convert HTMLs to EPUB
```bash
cargo run -- --convert
# Created EPUBs can be found in epub folder
```


#### Inside Kobo use KOReader.
Open Book and from bottom right-most settings, enable Embedded Style and Embedded Fonts.




Credits:
- All Data sourced from: https://www.quran.com
- IndoPak font: https://github.com/marwan/indopak-quran-text