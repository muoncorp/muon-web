+++
title = "Flutter에서 Rinf 이용해서 비즈니스 로직을 Rust로 작성하기"

[taxonomies]
categories = ["app", "web"]
tags = ["axum", "flutter", "rinf", "reqwest", "wasm"]
authors = ["ceo"]
+++

Flutter를 이용하면 Android, iOS와 같은 모바일 플랫폼과 Linux, Windows, macOS와 같은 데스크탑 앱 그리고 Web까지 지원하는 앱을 한번에 작성할 수 있다. 이것이 아마도 Flutter의 가장 큰 장점이 아닌가 싶다.
Flutter로 코딩을 하면서 느꼈던 것 중 하나는 화면 UI 코드와 상태 관리를 위한 비즈니스 로직들이 뒤섞여 점점 코드가 복잡해진다는 점이었다.

<!-- more -->

특히 API Server는 Rust로 작성되고, Flutter 앱에서 API를 사용하려면 dart로 model을 재정의해야 하는 문제가 있었다. 그래서 찾은 해결책은 Flutter 앱의 비즈니스 로직은 모두 Rust로 작성할 수 있게 해주는 [Rinf](https://rinf.cunarist.com/)을 적용하는 것이었다.

물론 Native로 동작하는 Rust 코드와 Dart 사이에 데이터를 주고 받기 위해서는 protobuf를 통해서 이루어지기 때문에 proto 파일을 만드는 작업이 필요하다. 그렇지만 App에서는 최종적으로 UI에 표시하기 위한 상호작용들만 정의하면 되기 때문에 내부에 세부적인 것까지 모두 proto로 정의할 필요는 없다.

이 글에서는 간단한 숫자를 카운트하고 보여주는 앱을 만들 것이다.
실제적으로 숫자를 카운트하는 것은 Rust의 axum 웹 프레임워크를 이용하여 제작된 API 서버가 담당한다.
그리고 Rinf를 통해 App의 Native에서는 Rust의 reqwest라는 API 클라이언트를 이용하여 API 서버와 통신을 하고, Dart와는 protobuf를 통해 이벤트를 주고 받게 된다.

이 글을 읽고 있는 분들은 Rust와 Flutter를 이용한 코딩에 어느 정도 있숙하다는 것을 가정하고 설명하겠다.

아래는 전체 프로젝트의 루트 폴더를 만드는 작업이다.

```sh
$ mkdir axum-rinf-flutter-example
$ cd axum-rinf-flutter-example
$ git init -b main
```

프로젝트 루트 폴더에 Cargo workspace를 만든다.

*Cargo.toml*
```toml
[workspace]
resolver = "2"
members = []
```

Cargo workspace를 만드는 이유는 Rust로 작성된 여러 프로젝트들을 빌드할 때 하나의 target 폴더를 만들게 된다.

*.gitignore*
```
.vscode/
target/
```

REDAME.md 파일도 만들어 준다.

*README.md*
```md
# axum-rinf-flutter-example
```

API 서버와 모델 프로젝트를 생성한다.
models 프로젝트는 Flutter의 native rust 프로젝트들에서 같이 사용된다.

```
$ cargo new api-server
$ cargo new --lib models
```

*Cargo.toml*
```toml
[workspace]
resolver = "2"
members = ["api-server", "models"]
```

models 프로젝트에 serde 의존성을 추가해준다.
`cargo add` 명령을 사용하기 위해서는 `cargo-edit`가 필요하니 설치를 하지 않았다면 설치하여 준다.

```sh
$ cd models
$ cargo install cargo-edit
$ cargo add serde --features=derive
```

*models/Cargo.toml*
```toml
[package]
name = "models"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0.197", features = ["derive"] }
```

Counter 모델을 정의하여 준다.

models/src/lib.rs
```rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Counter {
    pub number: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self { number: 0 }
    }

    pub fn increment(&mut self) {
        self.number += 1;
    }

    pub fn get(&self) -> i32 {
        self.number
    }

    pub fn set(&mut self, number: i32) {
        self.number = number;
    }
}

#[cfg(test)]
mod tests {
    use crate::Counter;

    #[test]
    fn test_counter() {
        let mut counter = Counter::new();
        assert_eq!(counter.get(), 0);
        counter.increment();
        assert_eq!(counter.get(), 1);
        counter.increment();
        assert_eq!(counter.get(), 2);
        counter.set(0);
        assert_eq!(counter.get(), 0);
    }
}
```

코드가 제대로 작성되었는지 테스트 코드를 실행하여 확인한다.

```sh
$ cd models
$ cargo test
```

이번엔 api-server를 작성한다. Rust로 작성된 웹 프레임워크 중 가장 많이 추천되는 axum으로 사용하였다.

```sh
$ cd api-server
$ cargo add models --path=../models
$ cargo add axum tracing tracing-subscriber thiserror
$ cargo add tokio --features=full
$ cargo add tower-http --features=cors
$ cargo add serde --features=serde
$ cargo add --dev axum-test
```

*api-server/Cargo.toml*
```toml
[package]
name = "api-server"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7.5"
axum-test = "14.8.0"
models = { version = "0.1.0", path = "../models" }
serde = { version = "1.0.197", features = ["derive"] }
thiserror = "1.0.58"
tokio = { version = "1.37.0", features = ["full"] }
tower-http = { version = "0.5.2", features = ["cors"] }
tracing = "0.1.40"
tracing-subscriber = "0.3.18"

[dev-dependencies]
axum-test = "14.8.0"
```

api-server의 메인 코드를 작성한다.

*api-server/src/main.rs*
```rs
use std::sync::Arc;

use axum::{
    extract::State,
    http::{self, header::InvalidHeaderName, HeaderName, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, put},
    Json, Router,
};
use models::Counter;
use serde::Serialize;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<Counter>>, // Wrap it with Arc and Mutex to share between threads.
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid header name: {0}")]
    InvalidHeaderName(#[from] InvalidHeaderName),
    #[error("axum error: {0}")]
    AxumError(#[from] axum::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let error = self.to_string();
        let status_code = match self {
            Error::InvalidHeaderName(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::AxumError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, Json(ErrorResponseBody { error })).into_response()
    }
}

#[derive(Serialize, Debug)]
struct ErrorResponseBody {
    error: String,
}

type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger.
    tracing_subscriber::fmt::init();

    // Start the server.
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3000))
        .await
        .unwrap();
    axum::serve(listener, create_app()?).await.unwrap();

    Ok(())
}

// Create an app by defining routes.
fn create_app() -> Result<Router> {
    let app_state = AppState {
        counter: Arc::new(Mutex::new(Counter { number: 0 })),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/counter", get(get_counter))
        .route("/counter", put(set_counter))
        .with_state(app_state)
        .layer(cors_layer()?);

    Ok(app)
}

// This is necessary to use it on a web built with Flutter.
fn cors_layer() -> Result<CorsLayer> {
    Ok(CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![
            http::header::AUTHORIZATION,
            http::header::CONTENT_TYPE,
            HeaderName::try_from("x-response-content-type")?,
        ]))
}

// Increments the counter every time it runs.
async fn get_counter(State(app_state): State<AppState>) -> Result<Json<Counter>> {
    let mut counter = app_state.counter.lock().await;
    let json = Json(counter.clone());
    counter.increment();
    Ok(json)
}

// Set counter number.
async fn set_counter(
    State(app_state): State<AppState>,
    Json(new_counter): Json<Counter>,
) -> Result<StatusCode> {
    let mut counter = app_state.counter.lock().await;
    counter.set(new_counter.get());
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use models::Counter;

    use crate::{create_app, Result};

    #[tokio::test]
    async fn test_hello_world() -> Result<()> {
        let server = TestServer::new(create_app()?).unwrap();
        let response = server.get("/").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(response.text(), "Hello, world!");
        Ok(())
    }

    #[tokio::test]
    async fn test_counter() -> Result<()> {
        let server = TestServer::new(create_app()?).unwrap();

        let response = server.get("/counter").await;
        let counter: Counter = response.json();
        assert_eq!(counter.get(), 0);

        let response = server.get("/counter").await;
        let counter: Counter = response.json();
        assert_eq!(counter.get(), 1);

        let mut counter = Counter::new();
        counter.set(100);

        let response = server.put("/counter").json(&counter).await;
        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

        let response = server.get("/counter").await;
        let counter: Counter = response.json();
        assert_eq!(counter.get(), 100);

        Ok(())
    }
}

```

역시 제대로 동작하는지 작성한 테스트 코드를 실행하여 확인한다.

```sh
$ cargo test
```

api-server를 실행하고 실제 HTTP Request를 보내 테스트를 해본다.

```sh
$ cd api-server
$ cargo run
```

HTTP Request를 보내기 위해 xh라는 툴을 사용한다. 유명한 curl 프로그램과 같은 역할을 한다.

```sh
$ cargo install xh
...

$ xh -b localhost:3000
Hello, world!

$ xh -b localhost:3000/counter
{
    "number": 0
}

$ xh -b localhost:3000/counter
{
    "number": 1
}
```

서버까지 제대로 동작되는지 확인이 됐으면 Flutter와 rinf를 사용하는 앱을 만든다.

```sh
$ flutter create app
$ cd app
$ flutter pub add rinf
$ cargo install rinf
$ rinf template
```

이렇게 앱 프로젝트를 생성하면 app 폴더에 Cargo workspace가 만들어지는데, 우리는 이미 상위 프로젝트 루트 폴더에 workspace를 만들어 놓았기 때문에 아래 내용들을 상위로 올릴 필요가 있다.

아래 생성된 파일은 삭제한다.

*app/Cargo.toml*
```toml
# This file is used for telling Rust-related tools
# where various Rust crates are.
# This also unifies `./target` output folder and
# various Rust configurations.

[workspace]
members = ["./native/*"]
resolver = "2"
```

```sh
$ rm app/Cargo.toml
```

프로젝트 루트의 Cargo.toml을 아래와 같이 수정한다.

*Cargo.toml*
```toml
[workspace]
resolver = "2"
members = ["api-server", "models", "app/native/*"]
```

그리고 rinf로 만들어진 필요없는 샘플 코드들을 삭제한다.

```sh
$ cd app/messages
$ rm -rf counter_number.proto fractal_art.proto sample_folder/
```

그리고 아래와 같이 proto 파일을 작성해준다.

*app/messages/counter.proto*
```proto
syntax = "proto3";
package counter;

// [RINF:DART-SIGNAL]
message SetCounter {
    int32 counter = 1;
}

// [RINF:RUST-SIGNAL]
message Counter {
    int32 number = 1;
}
```

`rinf message` 명령으로 rust와 dart에 사용할 수 있는 코드를 자동으로 생성해주도록 한다.

```sh
$ cd app
$ rinf message
```

app/native 밑에 sample_crate 프로젝트를 삭제한다.

```sh
$ cd app/native
$ rm -rf sample_crate
$ cd hub
$ rm sample_functions.rs
```

그리고 app/native/hub 프로젝트에서 sample_crate 의존성을 제거한다.

*app/native/hub/Cargo.toml*
```toml
[package]
# Do not change the name of this crate.
name = "hub"
version = "0.1.0"
edition = "2021"

[lib]
# `lib` is required for non-library targets,
# such as tests and benchmarks.
# `cdylib` is for Linux, Android, Windows, and web.
# `staticlib` is for iOS and macOS.
crate-type = ["lib", "cdylib", "staticlib"]

[dependencies]
rinf = "6.7.0"
prost = "0.12.3"
wasm-bindgen = "0.2.91"
tokio_with_wasm = "0.4.3"
```

마찬가지로 hub의 lib.rs에서 sample_functions를 사용하는 코드를 제거한다.

*app/native/hub/src/lib.rs*
```rs
//! This `hub` crate is the
//! entry point of the Rust logic.

// This `tokio` will be used by Rinf.
// You can replace it with the original `tokio`
// if you're not targeting the web.
use tokio_with_wasm::tokio;

mod messages;

rinf::write_interface!();

// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
    // Repeat `tokio::spawn` anywhere in your code
    // if more concurrent tasks are needed.
}
```

자 이제 api-server와 통신하기 위한 api-client를 app/native 밑에 생성해보자.

```sh
$ cd app/native
$ cargo new --lib api-client
$ cd api-client
$ cargo add models --path=../../../models
$ cargo add reqwest --no-default-features --features=json
$ cargo add thiserror
```

아래와 같이 코드를 추가한다. HOST 상수에 api-server가 실행되고 있는 IP 주소를 넣어주어야 한다. server와 앱이 같이 실행되는 환경이면 localhost로 지정해준다.

*app/native/api-client/src/lib.rs*
```rs
use models::Counter;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

// When running on mobile, you must specify the IP address.
// static HOST: &str = "http://<api-server_ip_addr>:3000";
static HOST: &str = "http://localhost:3000";

pub async fn get_counter() -> Result<Counter> {
    // `reqwest` supports all platforms, including web.
    let counter = reqwest::get(format!("{HOST}/counter"))
        .await?
        .json::<Counter>()
        .await?;
    Ok(counter)
}

pub async fn set_counter(counter: &Counter) -> Result<bool> {
    let response = reqwest::Client::new()
        .put(format!("{HOST}/counter"))
        .json(&counter)
        .send()
        .await?;
    Ok(response.status().is_success())
}
```

hub 프로젝트에서 dart와 rust의 상호작용이 일어나면 api-client 라이브러리를 사용하여 api-server와 통신을 하여 데이터를 주고 받도록 한다.

```sh
$ cd app/native/hub
$ cargo add models --path=../../../models
$ cargo add api-client --path=../api-client
```

*app/native/hub/lib/counter.rs*
```rs
use crate::messages;
use crate::tokio;
use messages::counter::*;
use rinf::debug_print;

pub async fn set_counter() {
    let mut receiver = SetCounter::get_dart_signal_receiver();
    while let Some(dart_signal) = receiver.recv().await {
        let set_counter = dart_signal.message;

        let mut counter = models::Counter::new();
        counter.set(set_counter.counter);

        if let Err(e) = api_client::set_counter(&counter).await {
            debug_print!("api_client::set_counter() error: {e}");
        }
    }
}

pub async fn counter() {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        match api_client::get_counter().await {
            Ok(counter) => {
                Counter {
                    number: counter.get(),
                }
                .send_signal_to_dart(None);
            }
            Err(e) => {
                debug_print!("api_client::get_counter() error: {e}");
            }
        }
    }
}
```

hub 프로젝트의 lib.rs에 set_counter와 counter 쓰레드를 만들어 실행되도록 한다.

*app/native/hub/src/lib.rs*
```rs
//! This `hub` crate is the
//! entry point of the Rust logic.

// This `tokio` will be used by Rinf.
// You can replace it with the original `tokio`
// if you're not targeting the web.
use tokio_with_wasm::tokio;

mod counter;
mod messages;

rinf::write_interface!();

// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
    // Repeat `tokio::spawn` anywhere in your code
    // if more concurrent tasks are needed.
    tokio::spawn(counter::set_counter());
    tokio::spawn(counter::counter());
}
```

Android 앱으로 실행한다면 네트워크를 사용하기 위해 아래와 같이 AndroidManifest.xml에 권한을 추가하여야 한다.

*app/android/app/src/main/AndroidManifest.xml*
```xml
<manifest xmlns:android="http://schemas.android.com/apk/res/android">
    <uses-permission android:name="android.permission.INTERNET" />
    ...
</manifest>
```

macOS 앱으로 만든다고 하면 *.entitlements 파일들에 아래와 같은 내용이 추가되어야 한다.

```xml
<key>com.apple.security.network.client</key>
<true/>
```

마지막으로 Flutter UI 코드를 만들어줄 차례이다.
간단하게 현재 카운터를 보여주고 0으로 리셋해주는 버튼을 만들어준다.

*app/lib/main.dart*
```dart
import 'package:app/messages/counter.pb.dart';
import 'package:flutter/material.dart';
import './messages/generated.dart';

void main() async {
  await initializeRust();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'axum-rinf-flutter-example',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'axum-rinf-flutter-example'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            StreamBuilder(
                stream: Counter.rustSignalStream,
                builder: (context, snapshot) {
                  final rustSignal = snapshot.data;
                  if (rustSignal == null) {
                    return const Text("Nothing received yet");
                  }
                  final counter = rustSignal.message;
                  final currentNumebr = counter.number;
                  return Text(
                    currentNumebr.toString(),
                    style: const TextStyle(fontSize: 40),
                  );
                }),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          SetCounter(counter: 0).sendSignalToRust(null);
        },
        tooltip: 'Reset counter number',
        child: const Icon(Icons.restart_alt),
      ),
    );
  }
}
```

앱이 동작하기 위해서는 먼저 api-server를 실행한다.

```sh
$ cd api-server
$ cargo run
```

Flutter 앱을 빌드하고 실행한다.

```sh
$ cd app
$ flutter run
```

웹으로 빌드하기 위해서는 아래와 같이 해준다.

```sh
$ rinf wasm --release
$ flutter build web
```

아래는 앱이 실행되고 있는 화면을 녹화한 것이다.

<video src="./app-record.mp4" width="504" height="388" controls></video>

소스 코드는 아래 [github](https://github.com/muonceo/axum-rinf-flutter-example)에 올려놓았다.




