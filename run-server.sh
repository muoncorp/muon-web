#!/bin/sh

run_server () {
    killall target/release/muon-website-server
    cd backend
    RUST_LOG="debug" nohup ~/.cargo/bin/cargo run --release >> /tmp/muon-website-server.log &
}

run_server
