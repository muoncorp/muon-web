#!/bin/sh

update_frontend () {
    (cd frontend && zola build) 
}

run_server () {
    killall target/release/muon-website-server
    cd backend
    RUST_LOG="debug" nohup ~/.cargo/bin/cargo run --release >> /tmp/muon-website-server.log &
}

update_frontend && run_server
