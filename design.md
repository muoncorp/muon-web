# Design notes

Notes for the redesigned pages. Home is done, the rest still use Bulma.

Tokens live in `frontend/sass/site.scss`. If you change something there, update this file too.

## Colors

From the brand package (muon-ci `colors/color.js`) and the logo.

| Token | Value | Used for |
| --- | --- | --- |
| `--ci-navy` | `#022c4c` | text, header, footer, ODM panel |
| `--ci-paper` | `#fdfdfd` | background, text on navy |
| `--ci-pale` | `#b5c3d9` | lines, tinted band, secondary text on navy |
| `--ci-yellow` | `#ffd012` | accent (yellow set) |
| `--ci-blue` | `#2c89f3` | accent (blue set), same blue as the logo mark |

Other colors are mixed from these in the token block. Don't write color values anywhere else.

Accent is set with `extra.accent` in `config.toml`, `yellow` or `blue`. We haven't picked one yet.

- yellow: yellow button, navy text, navy border. Yellow on white is 1.45:1 so never use it for text on white.
- blue: the logo blue is 3.45:1 on white, not enough for button text. Buttons use a darker mix (78% blue + 22% navy, about 4.7:1). The original blue is only used on navy.

Logo: white one for yellow, blue mark for blue.

`data-theme="dark"` tokens exist but nothing turns them on. Only contrast was checked.

## Fonts

Pretendard 400 / 600 / 700, self hosted in `frontend/static/fonts` (subset files from the Pretendard release, OFL).

They are about 265KB each, so `site.html` loads them after the first paint. Don't move the `@font-face` rules back into `site.scss`, Lighthouse drops from 99 to 76.

Sizes at 1440px: h1 72, h2 52, h3 25, lead 21, body 16.

`word-break: keep-all` is on for Korean. No uppercase labels, no small label above section titles.

## Layout

- container 1440px, gutter 64px (24px below 960px, 16px below 640px)
- 12 columns, 32px gap, tracks are `minmax(0, 1fr)`
- 120 to 176px between sections, spacing only from `--space-*`
- header is 72px, sticky. Below 960px the menu goes behind a button. Without JS it stays open.

## Other

- Radius is 4px. Only the hero photos use the logo mark shape, don't reuse it.
- Hover only changes color (120ms). Nothing animates on scroll.
- Keep the section ids (`business-field`, `company-tech`, `products`) and URLs.
- Real product photos with alt text only.
- Product photos link to the original file, the video plays in place.

## Moving another page over

1. `{% extends "base.html" %}` -> `{% extends "site.html" %}`
2. Swap Bulma classes for the ones in `site.scss`. Add what's missing.
3. Build and check the pages you didn't touch.

Remove Bulma, Font Awesome and the carousel after all pages are moved.
