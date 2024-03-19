#!/bin/sh
kill $(ps ax | grep muon-website-server | head -n 1 | awk '{ print $1 }')
cd dist/backend
RUST_LOG="debug" nohup ./target/release/muon-website-server >> /tmp/muon-website-server.log &
