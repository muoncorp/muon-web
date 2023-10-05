# Server

## Install packages for development

```bash
cargo install systemfd
cargo install cargo-watch
```

## Run

```bash
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

or

```bash
./run-server-watch.sh
```
