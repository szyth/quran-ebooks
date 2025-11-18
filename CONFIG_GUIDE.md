# Configuration Guide

The `quran-epubs` tool uses **separate JSON configuration files** for translations and tafsir.

## Quick Start

1. Configure `translation_config.json` and/or `tafsir_config.json` in the root directory (both files exist by default)
2. Run with the appropriate flag:
   - `cargo run -- --translations` (for translations)
   - `cargo run -- --tafsir` (for tafsir)
3. Convert the generated HTML files to EPUB using Calibre (see "Converting to EPUB" section below)

---

## Translation Configuration (`translation_config.json`)

### Structure

```json
{
  "start_surah": 1,
  "end_surah": 114,
  "arabic": {
    "script": "indopak"
  },
  "word_by_word": true,
  "translation": {
    "id": 20
  }
}
```

### Options Explained

#### `start_surah` (required)
- **Type**: Number (1-114)
- **Description**: The surah number to start from
- **Example**: `1` for Al-Fatihah

#### `end_surah` (required)
- **Type**: Number (1-114)
- **Description**: The surah number to end at
- **Example**: `114` for An-Nas
- **Note**: Must be >= `start_surah`

#### `arabic` (optional)
- **Type**: Object or `null`
- **Description**: Include Arabic text in your EPUB
- **Options**:
  - `null` - Do not include Arabic text
  - `{"script": "indopak"}` - Use Indo-Pak Nastaleeq script (traditional Pakistani/Indian style)
  - `{"script": "uthmani"}` - Use Uthmanic Hafs script (standard Arabic style)

**Example:**
```json
"arabic": {
  "script": "indopak"
}
```

#### `word_by_word` (required)
- **Type**: Boolean (`true` or `false`)
- **Description**: Include word-by-word English translation
- **Example**: `true`

#### `translation` (optional)
- **Type**: Object or `null`
- **Description**: Include full verse translation
- **Options**:
  - `null` - Do not include translation
  - `{"id": <number>}` - Include translation with specified ID

**Popular Translation IDs:**
- `20` - Sahih International (English)
- `131` - Dr. Mustafa Khattab, The Clear Quran (English)
- `85` - Mufti Taqi Usmani (English)
- `57` - Muhammad Asad (English)
- `19` - Tafsir al-Jalalayn (Arabic)

**Example:**
```json
"translation": {
  "id": 131
}
```

### Translation Config Examples

#### 1. Full Quran with Everything
```json
{
  "start_surah": 1,
  "end_surah": 114,
  "arabic": {
    "script": "indopak"
  },
  "word_by_word": true,
  "translation": {
    "id": 20
  }
}
```

#### 2. Translation Only (No Arabic)
```json
{
  "start_surah": 1,
  "end_surah": 114,
  "arabic": null,
  "word_by_word": false,
  "translation": {
    "id": 131
  }
}
```

#### 3. Arabic Text Only (Uthmanic Script)
```json
{
  "start_surah": 1,
  "end_surah": 114,
  "arabic": {
    "script": "uthmani"
  },
  "word_by_word": false,
  "translation": null
}
```

#### 4. Single Surah with Word-by-Word
```json
{
  "start_surah": 2,
  "end_surah": 2,
  "arabic": {
    "script": "indopak"
  },
  "word_by_word": true,
  "translation": null
}
```

#### 5. Juz 30 (Last Juz)
```json
{
  "start_surah": 78,
  "end_surah": 114,
  "arabic": {
    "script": "indopak"
  },
  "word_by_word": true,
  "translation": {
    "id": 20
  }
}
```

### Validation Rules

The tool will validate your translation configuration and show errors if:

1. `start_surah` or `end_surah` is not between 1-114
2. `start_surah` is greater than `end_surah`
3. All content options (`arabic`, `word_by_word`, `translation`) are disabled
4. The JSON syntax is invalid

---

## Tafsir Configuration (`tafsir_config.json`)

### Structure

```json
{
  "start_surah": 1,
  "end_surah": 114,
  "resource_id": 168
}
```

### Options Explained

#### `start_surah` (required)
- **Type**: Number (1-114)
- **Description**: The surah number to start from
- **Example**: `1` for Al-Fatihah

#### `end_surah` (required)
- **Type**: Number (1-114)
- **Description**: The surah number to end at
- **Example**: `114` for An-Nas
- **Note**: Must be >= `start_surah`

#### `resource_id` (required)
- **Type**: Number
- **Description**: Tafsir resource ID from Quran.com

**Popular Tafsir Resource IDs:**
- `168` - Ma'arif al-Qur'an (Mufti Muhammad Shafi)
- Check `static/tafsirs.json` for more options

### Tafsir Config Examples

#### 1. Full Quran Tafsir
```json
{
  "start_surah": 1,
  "end_surah": 114,
  "resource_id": 168
}
```

#### 2. Single Surah Tafsir
```json
{
  "start_surah": 18,
  "end_surah": 18,
  "resource_id": 168
}
```

#### 3. Juz 30 Tafsir
```json
{
  "start_surah": 78,
  "end_surah": 114,
  "resource_id": 168
}
```

### Validation Rules

The tool will validate your tafsir configuration and show errors if:

1. `start_surah` or `end_surah` is not between 1-114
2. `start_surah` is greater than `end_surah`
3. The JSON syntax is invalid

---

## Usage

Both `translation_config.json` and `tafsir_config.json` exist by default in the root directory. You **must always specify** which one to run using a CLI flag.

### Generate Translations
```bash
cargo run -- --translations
```

### Generate Tafsir
```bash
cargo run -- --tafsir
```

### Generate Both
Run the commands separately:
```bash
# First generate translations
cargo run -- --translations

# Then generate tafsir
cargo run -- --tafsir
```

**Note:** You can only run ONE at a time. Cannot specify both `--translations` and `--tafsir` together.

---

## Output Files

After running `cargo run`, your generated files will be in the `output/` directory:

- **Translations**: `output/quran-surah-{start}-{end}.html`
- **Tafsir**: `output/{number}. {name}-{tafsir-name}.html`

### Converting to EPUB

Use Calibre's `ebook-convert` to convert HTML to EPUB:

```bash
mkdir -p ebooks
for f in output/*.html; do
  ebook-convert "$f" "ebooks/$(basename "${f%.html}.epub")" --disable-font-rescaling
done
```

---

## Tips

1. **Start Small**: Test with a single surah first (e.g., start_surah: 1, end_surah: 1)
2. **Independent Generation**: Translation and tafsir are completely independent - you can generate one without the other
3. **Check File Size**: Full Quran HTML files (and resulting EPUBs) can be large, especially with all options enabled
4. **Translation IDs**: Find more translation IDs at [quran.com](https://quran.com)
5. **Font Choice**:
   - Use `indopak` for traditional South Asian style (Nastaleeq)
   - Use `uthmani` for standard Arabic style (similar to printed Mushafs)
6. **HTML to EPUB**: This tool generates HTML files. Use Calibre's `ebook-convert` to create EPUB files (see "Converting to EPUB" section)

---

## Need Help?

- Both config files (`translation_config.json` and `tafsir_config.json`) exist by default in the root directory
- Ensure your `.env` file is set up with valid API credentials
- Review error messages - they'll tell you exactly what's wrong
- You must always specify `--translations` or `--tafsir` flag when running the tool
