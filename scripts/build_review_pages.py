"""Build the scholar review pages (docs/review/) from the Arabic-only eBook HTMLs.

    python3 scripts/build_review_pages.py

Reads  output/quran-surah-1-114-indopak.html and output/quran-surah-1-114-uthmani.html
Writes docs/review/indopak.html and docs/review/uthmani.html

The Quranic text is copied exactly as it is in the eBooks. This script only adds
navigation and labels around it: a surah menu, a "surah:ayah" label and a Report
button on each ayah. It never changes anything inside the Arabic text.
"""
import json
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SURAHS = json.load(open(os.path.join(ROOT, "static/quran_surahs.json"), encoding="utf-8"))
SCRIPTS = {"indopak": "IndoPak", "uthmani": "Uthmani"}

PAGE = """<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="robots" content="noindex">
<title>{label} Quran: review copy</title>
{style}
<style>
  body {{ margin: 0; background: #fff; color: #111; font-family: Georgia, serif; }}
  .toc {{ display: none; }}
  .bar {{
    position: sticky; top: 0; z-index: 5; background: #f6f4ef; border-bottom: 1px solid #e3dfd5;
    padding: 10px 16px; display: flex; flex-wrap: wrap; gap: 10px 16px; align-items: center;
    font: 15px/1.4 -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
  }}
  .bar strong {{ font-size: 1rem; }}
  .bar select {{ font: inherit; padding: 6px 8px; border: 1px solid #ccc; border-radius: 8px; max-width: 70vw; }}
  .bar a {{ color: #1f5c4a; font-weight: 600; }}
  .intro {{
    max-width: 820px; margin: 20px auto; padding: 0 16px;
    font: 16px/1.55 -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif; color: #333;
  }}
  .intro h1 {{ font-size: 1.5rem; margin: 0 0 8px; }}
  .intro p {{ margin: 0 0 10px; }}
  main {{ max-width: 820px; margin: 0 auto; padding: 0 16px 80px; }}
  .key {{
    font: 600 14px/1 -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
    color: #1f5c4a; text-decoration: none; margin-right: 10px;
  }}
  .report {{
    font: 13px/1 -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
    border: 1px solid #d9b98f; color: #7a4410; background: #fff7ed; border-radius: 999px;
    padding: 4px 10px; margin-left: 10px; cursor: pointer;
  }}
  .report:hover {{ background: #9a5b13; color: #fff; }}
  .container:target {{ background: #fff7ed; }}
  .cover {{ scroll-margin-top: 70px; }}
  .container {{ scroll-margin-top: 70px; }}
</style>
</head>
<body>
<div class="bar">
  <strong>{label} script: review copy</strong>
  <label>Go to surah <select id="jump" aria-label="Go to surah">{options}</select></label>
  <a href="{other}.html">Switch to {other_label}</a>
</div>
<div class="intro">
  <h1>{label} Quran: review copy</h1>
  <p>This is the complete Arabic text used in the {label} eBooks, exactly as it appears in them. Please read through it and report anything that looks wrong: a missing or misplaced mark, a wrong letter or word, a wrong ayah number, or a waqf sign in the wrong place.</p>
  <p>Each ayah is labelled <b>surah:ayah</b> (e.g. 2:27). Press <b>Report</b> next to an ayah to send an email about it, or write to <span id="addr"></span>. The "Page" markers show where each page of the 604-page Madani Mushaf begins, to help compare with a printed copy.</p>
</div>
<main>
{body}
</main>
<script>
  const addr = ["quranebooks.feedback", "gmail.com"].join("@");
  const a = document.createElement("a"); a.href = "mailto:" + addr; a.textContent = addr;
  document.getElementById("addr").append(a);
  document.addEventListener("click", (e) => {{
    const b = e.target.closest(".report");
    if (!b) return;
    const key = b.dataset.key;
    const subject = "Mistake in {label} Quran, ayah " + key;
    const body = "Script: {label}\\nAyah: " + key + "\\n\\nWhat is wrong:\\n\\n\\nWhat it should be:\\n\\n";
    location.href = "mailto:" + addr + "?subject=" + encodeURIComponent(subject) + "&body=" + encodeURIComponent(body);
  }});
  document.getElementById("jump").addEventListener("change", (e) => {{ location.hash = "surah-" + e.target.value; }});
</script>
</body>
</html>
"""


def build(script):
    src = open(os.path.join(ROOT, f"output/quran-surah-1-114-{script}.html"), encoding="utf-8").read()
    style_start = src.index("<style>")
    style_end = src.index("</style>") + len("</style>")
    # The fonts live in docs/previews/fonts; point the stylesheet there.
    style = src[style_start:style_end].replace("url('fonts/", "url('../previews/fonts/")
    body = src[style_end:]

    # Label each ayah with surah:ayah and give it an anchor and a Report button.
    # Only the header (outside the Arabic text) is touched.
    surah = 0

    def label(m):
        nonlocal surah
        cur = int(m.group(1))
        if cur == 1:
            surah += 1
        key = f"{surah}:{cur}"
        return (f'<a class="key" href="#a{surah}-{cur}">{key}</a>'
                f'<span class="ayah">{m.group(1)}/{m.group(2)}</span>'
                f'<button class="report" type="button" data-key="{key}">Report</button>')

    body = re.sub(r'<span class="ayah">(\d+)/(\d+)</span>', label, body)
    # Anchor each ayah's container so #a2-27 links work.
    counter = iter(re.findall(r'href="#(a\d+-\d+)"', body))
    body = re.sub(r'<div class="container">', lambda m: f'<div class="container" id="{next(counter)}">', body)

    options = "".join(f'<option value="{s["id"]}">{s["id"]}. {s["transliteratedName"]}</option>' for s in SURAHS)
    other = "uthmani" if script == "indopak" else "indopak"
    html = PAGE.format(label=SCRIPTS[script], style=style, options=options, other=other,
                       other_label=SCRIPTS[other], body=body)
    out = os.path.join(ROOT, f"docs/review/{script}.html")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    open(out, "w", encoding="utf-8").write(html)
    print(f"wrote {out}: {surah} surahs")


if __name__ == "__main__":
    for s in SCRIPTS:
        build(s)
