# Design notes

Every page uses this design now. Tokens live in `frontend/sass/site.scss`. If you change something there, update this file too.

## Colors

From the brand package (muon-ci `colors/color.js`) and the logo.

| Token | Value | Used for |
| --- | --- | --- |
| `--ci-navy` | `#022c4c` | text, header, footer, ODM panel |
| `--ci-paper` | `#fdfdfd` | background, text on navy |
| `--ci-pale` | `#b5c3d9` | lines, tinted band, secondary text on navy |
| `--ci-yellow` | `#ffd012` | accent (yellow set) |
| `--ci-blue` | `#2c89f3` | accent (blue set), same blue as the logo mark |

Other colors are mixed from these in the token block. Don't write color values anywhere else. The only other color is the black dim behind the large photo (`--color-dim`).

Accent is set with `extra.accent` in `config.toml`, `yellow` or `blue`. We haven't picked one yet.

- yellow: yellow button, navy text, navy border. Yellow on white is 1.45:1 so never use it for text on white.
- blue: the logo blue is 3.45:1 on white, not enough for button text. Buttons use a darker mix (78% blue + 22% navy, about 4.7:1). The original blue is only used on navy.

Logo: white one for yellow, blue mark for blue.

Light and dark: the default follows the OS setting (`extra.theme = "auto"` in `config.toml`). The button in the header switches and keeps the choice in localStorage (`theme`). Set `extra.theme` to `light` or `dark` to fix the default. The blog comment box (utterances) gets the same theme by message.

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
- Product photos are real, each with alt text.
- The ODM steps each have a picture, `static/images/odm/<accent>/<step number>.svg` (240x216). The colors are fixed inside the files, so there is one set for each accent.
- The business field pictures are in `static/images/fields`, 1024px square. The file name is the field title in lower case with dashes (`embedded.jpg`, `iot.jpg`, `os-porting.jpg`). A field without a file shows no picture.
- A product photo opens in a large view over the page (a `dialog`), closed with the X, Esc or a click outside the photo. When the product has several photos, its photos sit in a row that slides sideways: arrows and arrow keys with a mouse, a finger drag on a touch screen (no arrows there), and a count such as "1 / 2" shows under it. Without JS it links to the original file. The video plays in place.
- A product with several photos shows one at a time: dots under the photo, arrows on hover, swipe on touch.

## Old files

`base.html`, `style.scss`, the old macros and the Bulma and carousel files are still in the repo. No page loads them. Delete them once the new pages are live.

Font Awesome: the header icons are inline svg read from `sass/_vendor/fontawesome-free-6.4.2-web/svgs`, picked by the `icon` value in `config.toml`. Keep that folder. Its css and the webfonts are not used.

The history page content still has Bulma timeline class names. `site.scss` styles those names, so keep them if you edit the content. Put each year header before the entries of that year.
