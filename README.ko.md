# Muon Web

English: [README.md](README.md)

[muon.co](https://muon.co) 웹사이트의 소스 코드 저장소입니다.


## 빌드 방법

### 프론트엔드

웹사이트는 정적 사이트 생성기 [Zola](https://www.getzola.org)로 만들었습니다.

먼저 Zola를 설치합니다. [Zola 설치 안내](https://www.getzola.org/documentation/getting-started/installation/)

그다음 Zola로 프론트엔드를 빌드합니다.

```shell
> cd frontend
> zola build --base-url=http://127.0.0.1:1111/
```

### 백엔드

백엔드는 문의 메일을 보내는 데 씁니다.

Rust로 작성했으므로 먼저 [rustup](https://rustup.rs/)으로 Rust 개발 환경을 갖춰야 합니다.

그다음 cargo 명령으로 빌드하고 실행합니다.

```shell
> cd backend
> cargo build
> cargo run
```

[cargo-watch](https://crates.io/crates/cargo-watch)를 쓰면 소스를 고칠 때마다 자동으로 다시 빌드하고 실행합니다.

```shell
> cd backend
> cargo watch -x run
```

## 배포

릴리스용으로는 다음과 같이 빌드합니다.

```shell
> cd frontend
> zola build
> cd ../backend
> cargo build --release
```

Nginx 프록시는 다음과 같이 설정합니다.

```
server {
    server_name muon.co;

    location / {
        proxy_pass http://127.0.0.1:1111;
    }
}
```

HTTPS로 서비스하려면 [Certbot](https://certbot.eff.org/)을 사용합니다.

## 검색 엔진 대응 (SEO)

검색 엔진이 읽는 정보는 이미 페이지에 들어 있습니다. 항목별 위치는 다음과 같습니다.

| 항목 | 위치 |
| --- | --- |
| 사이트 설명, 메인 제목 뒤에 붙는 문구 | `frontend/config.toml`의 `description`, `extra.tagline` |
| 페이지별 설명 | 해당 페이지 front matter(파일 맨 위 `+++` 사이의 설정)의 `description`. 블로그 글에 이 값이 없으면 본문 앞 150자를 씁니다. |
| 제목, canonical, 공유 태그, 구조화 데이터(JSON-LD) | `frontend/templates/site.html`의 `<head>` |
| 공유 미리보기 이미지 (1200x630) | `frontend/static/images/og.jpg` |
| 크롤러 규칙 | `frontend/templates/robots.txt` |
| 사이트맵 | `frontend/templates/sitemap.xml`, 빌드하면 `/sitemap.xml` |
| 블로그 피드 | `frontend/config.toml`의 `generate_feeds`, 빌드하면 `/atom.xml` |
| AI 어시스턴트용 사이트 요약 | `frontend/static/llms.txt` |

나머지는 코드로 해결할 수 없습니다. 사이트를 배포한 뒤 아래 작업을 사람이 직접 해야 합니다.

### 1. 구글에 사이트 등록

1. [Google Search Console](https://search.google.com/search-console)에서 `https://muon.co`를 **URL 접두어** 속성으로 추가합니다.
2. 소유권 확인 방법으로 **HTML 태그**를 고릅니다. 그러면 `<meta name="google-site-verification" content="abc123...">` 같은 태그가 나옵니다.
3. 이 중 `content` 값만 `frontend/config.toml`에 넣습니다.

   ```toml
   [extra]
   google_site_verification = "abc123..."
   ```

4. 빌드하고 배포한 뒤 Search Console에서 **확인**을 누릅니다.
5. **Sitemaps** 메뉴에서 `sitemap.xml`을 제출합니다.
6. **URL 검사**에 `https://muon.co/`를 넣고 **색인 생성 요청**을 누릅니다.

DNS 레코드로 확인하는 **도메인** 속성을 쓴다면 2~4번은 건너뜁니다. 설정값을 비워 두면 태그도 출력되지 않습니다.

### 2. 네이버에 사이트 등록

국내 검색은 네이버 비중이 크므로 네이버에도 등록합니다.

1. [네이버 서치어드바이저](https://searchadvisor.naver.com)의 웹마스터 도구에서 `https://muon.co`를 등록합니다(사이트 등록).
2. 소유확인 방법으로 **HTML 태그**를 고르고, `content` 값을 `frontend/config.toml`에 넣습니다.

   ```toml
   [extra]
   naver_site_verification = "abc123..."
   ```

3. 빌드하고 배포한 뒤 소유확인을 누릅니다.
4. 요청 메뉴의 사이트맵 제출에 `https://muon.co/sitemap.xml`을 넣습니다.
5. 요청 메뉴의 RSS 제출에 `https://muon.co/atom.xml`을 넣습니다.

### 3. 배포된 사이트 점검

처음 배포한 뒤 한 번 확인하고, `site.html`의 `<head>`를 고칠 때마다 다시 확인합니다.

- [리치 결과 테스트](https://search.google.com/test/rich-results): 메인 주소와 블로그 글 주소 하나를 넣어 봅니다. 오류가 없어야 합니다.
- [Schema Markup Validator](https://validator.schema.org): 같은 두 주소를 넣습니다. 모든 페이지에 Organization과 WebSite가, 블로그 글에는 BlogPosting이 보여야 합니다.
- 공유 미리보기: 카카오톡이나 Slack에 링크를 붙여 넣고 제목, 설명, 이미지를 확인합니다. 페이스북과 카카오는 예전 미리보기를 저장해 두므로, `og.jpg`나 제목을 바꾼 뒤에는 [Facebook 공유 디버거](https://developers.facebook.com/tools/debug/)와 [카카오 공유 디버거](https://developers.kakao.com/tool/debugger/sharing)에서 캐시를 지웁니다.
- `https://muon.co/robots.txt`, `/sitemap.xml`, `/atom.xml`, `/llms.txt`가 각각 열리는지 확인합니다.

몇 주 뒤에는 Search Console의 **페이지**(색인 생성)와 **실적**, 네이버 서치어드바이저의 검색 노출을 살펴봅니다. 등록한다고 순위가 보장되지는 않습니다. 이 리포트에서는 검색 엔진이 무엇을 수집했는지 볼 수 있습니다.

### 4. 업체 정보 등록

웹사이트와 별도로 관리하는 정보입니다. 등록해 두면 지도와 검색 결과 옆 정보 패널에 회사가 나옵니다.

- [Google 비즈니스 프로필](https://business.google.com)
- [네이버 스마트플레이스](https://smartplace.naver.com)

회사명, 주소, 전화번호는 사이트 푸터와 똑같이 적습니다. 검색 엔진은 이 세 가지로 업체 정보와 사이트를 연결합니다.

### 5. 사이트로 돌아오는 링크 걸기

유튜브 채널 설명, 네이버 스토어 프로필, GitHub 조직 프로필에 `https://muon.co`를 넣습니다. 구조화 데이터에 이 세 곳을 회사 계정으로 적어 두었습니다. 각 계정에서도 사이트로 링크가 걸려 있어야 같은 회사라는 것이 확인됩니다.

### 회사 정보가 바뀔 때

회사명, 주소, 전화번호는 여러 파일에 적혀 있습니다. 바꿀 때는 모두 함께 고칩니다.

- `frontend/templates/macros/site_footer.html` (푸터)
- `frontend/templates/site.html` (JSON-LD의 `Organization` 블록)
- `frontend/static/llms.txt`
- `frontend/templates/about/section.html`, `frontend/templates/contact/section.html` (페이지에 보이는 주소와 전화번호)

4번에서 등록한 업체 정보도 같이 수정합니다.

### 글을 쓸 때

- 새 블로그 글에는 front matter에 `description`을 넣습니다. 글이 어떤 질문에 답하는지 한두 문장으로 씁니다.
- 글에 넣는 이미지에는 모두 `alt`를 답니다.
- 검색 엔진과 AI 어시스턴트는 고객이 실제로 묻는 질문의 답을 주로 인용합니다. 예를 들면 개발 의뢰 방법, 소요 기간, 최소 수량, 지원하는 MCU와 운영체제입니다. 사이트에는 아직 이런 페이지가 없습니다. 답변이 준비되면 페이지로 추가하고 FAQ 구조화 데이터를 붙이면 됩니다.

### AI 크롤러

`frontend/templates/robots.txt`는 AI 검색·어시스턴트 크롤러가 사이트를 읽도록 허용합니다. 특정 크롤러를 막으려면 공용 그룹에서 해당 줄을 빼고 그룹을 따로 만듭니다.

```
User-agent: GPTBot
Disallow: /
```

### 공유 이미지 교체

공유 이미지를 바꿀 때는 파일명 `og.jpg`와 크기 1200x630을 그대로 유지합니다.
