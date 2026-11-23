# Muon Web

한국어: [README.ko.md](README.ko.md)

This repository contains the source code for the [muon.co](https://muon.co) website.


## How to build

### Frontend

Our website was created using [Zola](https://www.getzola.org), one of the simple site generators.

First, install zola. [Zola Installation](https://www.getzola.org/documentation/getting-started/installation/)

Then, build the frontend using zola.

```shell
> cd frontend
> zola build --base-url=http://127.0.0.1:1111/
```

### Backend

The backend is used to send contact emails.

Our backend is written in the Rust language, you must first create a Rust development environment with [rustup](https://rustup.rs/).

Then, build and run it with the cargo command.

```shell
> cd backend
> cargo build
> cargo run
```

Alternatively, you can use [cargo-watch](https://crates.io/crates/cargo-watch) to automatically rebuild and run when the sources are modified.

```shell
> cd backend
> cargo watch -x run
```

## Deployment

This is how to build for release.

```shell
> cd frontend
> zola build
> cd ../backend
> cargo build --release
```

Use Nginx to set proxy settings as follows.

```
server {
    server_name muon.co;

    location / {
        proxy_pass http://127.0.0.1:1111;
    }
}
```

To service over HTTPS, use [Certbot](https://certbot.eff.org/).

## Search engines (SEO)

The pages already carry what a search engine reads. This is where each part lives.

| What | Where |
| --- | --- |
| Site description, home title tagline | `description` and `extra.tagline` in `frontend/config.toml` |
| Description of one page | `description` in that page's front matter. A blog post without one uses the first 150 characters of its text. |
| Title, canonical, share tags, structured data (JSON-LD) | `<head>` of `frontend/templates/site.html` |
| Share preview image (1200x630) | `frontend/static/images/og.jpg` |
| Crawler rules | `frontend/templates/robots.txt` |
| Sitemap | `frontend/templates/sitemap.xml`, built to `/sitemap.xml` |
| Blog feed | `generate_feeds` in `frontend/config.toml`, built to `/atom.xml` |
| Site summary for AI assistants | `frontend/static/llms.txt` |

The rest cannot be done from the code. Someone has to do the steps below by hand, after the site is deployed.

### 1. Register the site with Google

1. Open [Google Search Console](https://search.google.com/search-console) and add `https://muon.co` as a **URL prefix** property.
2. Choose the **HTML tag** way of verifying. Google shows a tag like `<meta name="google-site-verification" content="abc123...">`.
3. Copy only the `content` value into `frontend/config.toml`:

   ```toml
   [extra]
   google_site_verification = "abc123..."
   ```

4. Build, deploy, then press **Verify** in Search Console.
5. Go to **Sitemaps** and submit `sitemap.xml`.
6. Use **URL inspection** on `https://muon.co/` and press **Request indexing**.

If you verify with a DNS record instead (a **Domain** property), skip steps 2 to 4. The config value stays empty and no tag is printed.

### 2. Register the site with Naver

Most searches in Korea go through Naver, so do this one too.

1. Open [Naver Search Advisor](https://searchadvisor.naver.com), go to 웹마스터 도구, and add `https://muon.co` (사이트 등록).
2. Choose **HTML 태그** for 소유확인. Copy the `content` value into `frontend/config.toml`:

   ```toml
   [extra]
   naver_site_verification = "abc123..."
   ```

3. Build, deploy, then press 소유확인.
4. Under 요청, submit the sitemap (사이트맵 제출): `https://muon.co/sitemap.xml`
5. Under 요청, submit the feed (RSS 제출): `https://muon.co/atom.xml`

### 3. Check the deployed site

Do this once after the first deploy, and again after changing the `<head>` of `site.html`.

- [Rich Results Test](https://search.google.com/test/rich-results): paste the home address and one blog post address. It should report no errors.
- [Schema Markup Validator](https://validator.schema.org): same two addresses. Expect Organization and WebSite on every page, BlogPosting on a post.
- Share preview: paste a link into KakaoTalk or Slack and check the title, description and image. Facebook and Kakao keep old previews, so after changing `og.jpg` or a title, clear them with the [Facebook Sharing Debugger](https://developers.facebook.com/tools/debug/) and the [Kakao sharing debugger](https://developers.kakao.com/tool/debugger/sharing).
- Open `https://muon.co/robots.txt`, `/sitemap.xml`, `/atom.xml` and `/llms.txt` and see that each one loads.

After a few weeks, look at **Pages** (indexing) and **Performance** in Search Console, and at 검색 노출 in Naver Search Advisor. Registration does not promise a ranking; these reports are how you see what was picked up.

### 4. Register the business listing

These are separate from the website and show the company on maps and in the side panel of search results.

- [Google Business Profile](https://business.google.com)
- [Naver Smart Place](https://smartplace.naver.com) (네이버 스마트플레이스)

Use exactly the same company name, address and phone number as the footer of the site. Search engines match the listing to the site by these.

### 5. Link back to the site

Put `https://muon.co` in the YouTube channel description, the Naver store profile and the GitHub organization profile. These three are listed as the company's accounts in the structured data, and a link back from each confirms it.

### When company details change

The company name, address and phone number are written in several files. Change all of them together:

- `frontend/templates/macros/site_footer.html` (footer)
- `frontend/templates/site.html` (the `Organization` block in the JSON-LD)
- `frontend/static/llms.txt`
- `frontend/templates/about/section.html` and `frontend/templates/contact/section.html` (address and phone shown on the pages)

Then update the business listings from step 4 as well.

### When writing content

- Give a new blog post a `description` in its front matter. One or two sentences that say what the post answers.
- Give every image in a post an `alt` text.
- Questions customers actually ask (how to request development, how long it takes, minimum quantity, supported MCUs and operating systems) are what search engines and AI assistants quote. The site has no such page yet. When the answers are ready, add them as a page and they can be marked up as an FAQ.

### AI crawlers

`frontend/templates/robots.txt` lets AI search and assistant crawlers read the site. To keep one out, take its line out of the shared group and give it its own:

```
User-agent: GPTBot
Disallow: /
```

### Share image

To replace the share image, keep the file name `og.jpg` and the size 1200x630.
