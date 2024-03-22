# Muon Webite

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